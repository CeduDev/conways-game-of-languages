use serde::{Deserialize, Serialize};

pub const PATTERNS: [&str; 8] = [
    "Blinker",
    "Toad",
    "Beacon",
    "Pulsar",
    "Penta Decathlon", 
    "Glider", 
    "Glider Gun", 
    "Bunnies"
];

#[derive(Debug, Deserialize, Serialize)]
pub struct Pattern {
    pub name: String,
    pub alive_cells: Vec<(u16, u16)>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Patterns {
    pub patterns: Vec<Pattern>
}

impl Pattern {
  pub fn get_max_alive(&self) -> (u16, u16) {
    let mut max_x = 0;
    let mut max_y = 0;
    for p in self.alive_cells.iter() {
      if p.1 > max_y { max_y = p.1 }
      if p.0 > max_x { max_x = p.0 }
    }

    (max_x, max_y)
  }
}