use std::{io::{self, stdout, Stdout, Write}, time::Duration, thread};
use crossterm::{
    execute, queue,
    style::{self}, cursor, terminal
};

use crate::alive_cells::AliveCells;

pub struct Conway {
  height: u16,
  width: u16,
  stdout: Stdout,
  text: String,
  alive_cells: AliveCells,
  alive: String,
  dead: String,
  generations: u16,
  delay: Duration
}

impl Conway {
  pub fn init(&mut self) -> io::Result<()> {
    execute!(self.stdout, terminal::Clear(terminal::ClearType::All))?;

    queue!(
      self.stdout,
      cursor::MoveTo((self.width - self.text.len() as u16) / 2, 0),
      style::Print(&self.text)
    )?;

    // Grid
    for y in 1..self.height {
        for x in 0..self.width {
            // in this loop we are more efficient by not flushing the buffer.
            if self.alive_cells.cells.contains(&(x, y)) {
                queue!(
                    self.stdout, 
                    cursor::MoveTo(x,y), 
                    style::Print(self.alive.as_str())
                )?;
            } else {
                queue!(
                    self.stdout, 
                    cursor::MoveTo(x,y), 
                    style::Print(self.dead.as_str())
                )?;                
            }
        }
    }

    self.stdout.flush()?;

    Ok(())
  }

  pub fn run(&mut self) -> io::Result<()> {
    self.init()?;

    for _ in 0..self.generations {
      self.alive_cells.evolve();

      for c in 0..self.alive_cells.cells.len() - 1 {
          let cell = self.alive_cells.cells[c];
          queue!(
            self.stdout,
            cursor::MoveTo(cell.0, cell.1),
            style::Print(self.alive.as_str())
          )?;
      }      

      self.stdout.flush()?;

      thread::sleep(self.delay)
    }


    Ok(())
  }
}

impl Default for Conway {
  fn default() -> Self {
      Conway {
        height: 20,
        width: 40,
        stdout: stdout(),
        text: "Blinker Pattern".to_string(),
        alive_cells: AliveCells::new([(8, 7), (8, 8), (8, 9)].to_vec()),
        alive: "♥".to_string(),
        dead: "‧".to_string(),
        generations: 10,
        delay: Duration::from_secs_f32(0.2)
      }
  }
}