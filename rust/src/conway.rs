use std::{io::{self, stdout, Stdout, Write}, thread, time::Duration};
use crossterm::{
    execute, queue,
    style::{self}, cursor, terminal
};

#[repr(u8)]
#[derive(PartialEq)]
enum Cell {
  DEAD = 0,
  ALIVE = 1,
}

pub struct Conway {
  height: u16,
  width: u16,
  stdout: Stdout,
  text: String,
  cells: Vec<Cell>,
  generations: u16,
  delay: Duration
}

impl Conway {
  pub fn new(width: u16, height: u16, mut alive_cells: Vec<(u16, u16)>, text: String, generations: u16, delay: Duration) -> Self {
    let alive_cells_index: Vec<u16> = alive_cells.
      iter_mut().
      map(|i| i.1 * width + i.0)
      .collect();

    let cells: Vec<Cell> = (0..width * height)
      .map(|i| {
        if alive_cells_index.contains(&i) {
          Cell::ALIVE
        } else {
          Cell::DEAD
        }
      })
      .collect();

    Conway {
        height,
        width,
        stdout: stdout(),
        text,
        cells,
        generations,
        delay
      }
  }

  fn get_cells_index(&self, row: u16, col: u16) -> usize {
    (row * self.width + col) as usize
  }

  // Print all cells
  fn print(&mut self) -> io::Result<()> {
     // Grid
    for row in 1..self.height {
      for col in 0..self.width {
        let idx = self.get_cells_index(row, col);
        let cell = &self.cells[idx];
        // in this loop we are more efficient by not flushing the buffer.
        queue!(
            self.stdout, 
            cursor::MoveTo(col,row), 
            style::Print(if *cell == Cell::DEAD { "‧".to_string() } else { "♥".to_string() })
        )?;
      }
    }

    self.stdout.flush()?;

    Ok(())
  }

  // Print the pattern text
  pub fn init(&mut self) -> io::Result<()> {
    execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(
      self.stdout,
      cursor::MoveTo((self.width - self.text.len() as u16) / 2, 0),
      style::Print(&self.text)
    )?;

    self.print()?;

    Ok(())
  }

  // Run the game
  pub fn run(&mut self) -> io::Result<()> {
    for _ in 0..self.generations {
      // self.alive_cells.evolve();
      self.print()?;
      thread::sleep(self.delay)
    }


    Ok(())
  }
}