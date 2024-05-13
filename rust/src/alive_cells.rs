pub struct AliveCells {
  pub cells: Vec<(u16, u16)>,
  neighbords_delta: Vec<(i16, i16)>
}

impl AliveCells {
  pub fn new(cells: Vec<(u16, u16)>) -> Self {
    AliveCells { 
      cells,
      neighbords_delta: [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1)
      ].to_vec()
    }
  }

  pub fn evolve(&self){
    return;
  }
}