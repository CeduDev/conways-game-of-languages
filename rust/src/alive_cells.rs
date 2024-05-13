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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blinker_works() {
        let blinker = AliveCells::new([(8, 7), (8, 8), (8, 9)].to_vec());
        assert_eq!(blinker.cells, [(8, 7), (8, 8), (8, 9)].to_vec());
        blinker.evolve();
        assert_eq!(blinker.cells, [(7, 8), (8, 8), (9, 8)].to_vec());
        // assert_eq!(result, 4);
    }
}