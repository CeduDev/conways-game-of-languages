use std::{cmp::min, io::{self, stdout, Stdout, Write}, thread, time::Duration};
use crossterm::{
    execute, queue,
    style::{self}, cursor, terminal
};

#[repr(u8)]
#[derive(PartialEq, Clone, Debug, Copy)]
pub enum Cell {
  DEAD = 0,
  ALIVE = 1,
}

const DELTAS: [(i8, i8); 8] = [
  (-1, -1),
  (-1, 0),
  (-1, 1),
  (0, -1),
  (0, 1),
  (1, -1),
  (1, 0),
  (1, 1),
];

pub struct Conway {
  pub height: u16,
  pub width: u16,
  stdout: Stdout,
  text: String,
  pub cells: Vec<Cell>,
  generations: u16,
  current_generation: u16,
  delay: Duration
}

impl Conway {
  pub fn new(width: u16, height: u16, mut init_cells: Vec<(u16, u16)>, text: String, generations: u16, delay: Duration) -> Self {
    let init_cells_index: Vec<u16> = init_cells.
      iter_mut().
      map(|i| i.1 * width + i.0)
      .collect();

    let cells: Vec<Cell> = (0..width * height)
      .map(|i| {
        if init_cells_index.contains(&i) {
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
        current_generation: 0,
        delay
      }
  }

  pub fn get_cells_index(&self, row: u16, col: u16) -> usize {
    (row * self.width + col) as usize
  }

  // Print all cells
  fn print(&mut self) -> io::Result<()> {
    let hud_text = self.text.clone() + " Pattern, Generation: " + self.current_generation.to_string().as_str();

    queue!(
      self.stdout,
      cursor::MoveTo( (self.width - min(hud_text.len(), self.width as usize) as u16) / 2, self.height),
      style::Print(hud_text)
    )?;

    for row in 0..self.height {
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

  // Get the alive neighbors for a Cell at (row, col)
  pub fn get_live_neighbors(&self, row: u16, col: u16) -> u8 {
    let mut count = 0;
    for (drow, dcol) in DELTAS {
      // Don't check cases when the position is outside the grid, i.e. edge cases
      if (row as i8 + drow) < 0 || 
      (col as i8 + dcol) < 0 ||
      (row as i8 + drow) > (self.height - 1) as i8 ||
      (col as i8 + dcol) > (self.width - 1) as i8
      { continue }

      // Convert to u16 as we know for sure now that the values are 0 or more
      let r = (row as i8 + drow) as u16;
      let c = (col as i8 + dcol) as u16;

      let n_index = self.get_cells_index(r, c);
      let cell = &self.cells[n_index];

      if *cell == Cell::ALIVE { count += 1 }
    }
    
    count
  }

  pub fn evolve(&mut self) {
    let mut next_evolution: Vec<Cell> = self.cells.clone();
    for row in 0..self.height {
      for col in 0..self.width {
        let idx = self.get_cells_index(row, col);
        let cell = &self.cells[idx];
        let live_neighbors_count = self.get_live_neighbors(row, col);
                
        let next_cell = match (cell, live_neighbors_count) {
          (Cell::ALIVE, x) if x < 2 => Cell::DEAD,
          (Cell::ALIVE, 2) | (Cell::ALIVE, 3) => Cell::ALIVE,
          (Cell::ALIVE, x) if x > 3  => Cell::DEAD,
          (Cell::DEAD, 3) => Cell::ALIVE,
          (otherwise, _) => *otherwise
        };
        
        next_evolution[idx] = next_cell;
      }
    }
    
    self.cells = next_evolution;
  }

  
  // Run the game
  pub fn run(&mut self) -> io::Result<()> {
    execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;

    for _ in 0..self.generations {
      self.print()?;
      thread::sleep(self.delay);
      self.evolve();
      self.current_generation += 1;
    }
    
    Ok(())
  }
}