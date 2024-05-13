use std::time::Duration;

use conways_game_of_languages::{self, conway::{Cell, Conway}};

fn get_cell_vec_tuple(game: &Conway) -> Vec<(u16, u16)> {
    let mut res: Vec<(u16, u16)> = vec![];
    
    for row in 0..game.height {
      for col in 0..game.width {
        let idx = game.get_cells_index(row, col);
        let cell = &game.cells[idx];
        if *cell == Cell::ALIVE { res.push((col, row)) }
      }
    }

    res
  }

  #[test]
  fn blinker_works() {
    let init_vec: Vec<(u16, u16)> = [(1, 0), (1, 1), (1, 2)].to_vec();
    let mut game = Conway::new(
      3, 
      3,
      init_vec.clone(), 
      "Blinker Pattern".to_string(),
      10,
      Duration::from_secs_f32(0.2)
    );

    let c1: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    assert!(c1.iter().all(|item| init_vec.contains(item)));
    
    game.evolve();
    let c2: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    let e2: Vec<(u16, u16)> = [(0, 1), (1, 1), (2, 1)].to_vec();
    assert!(c2.iter().all(|item| e2.contains(item)));
    
    game.evolve();
    game.evolve();
    game.evolve();
    game.evolve();
    game.evolve();
    let c3: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    assert!(c3.iter().all(|item| init_vec.contains(item)));
  }

  #[test]
  fn toad_word() {
    let init_vec: Vec<(u16, u16)> = [(2, 2), (2, 3), (2, 4), (3, 1), (3, 2), (3, 3)].to_vec();
    let mut game = Conway::new(
      8, 
      8,
      init_vec.clone(), 
      "Toad Pattern".to_string(),
      10,
      Duration::from_secs_f32(0.2)
    );

    let c1: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    assert!(c1.iter().all(|item| init_vec.contains(item)));
    
    game.evolve();
    let c2: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    let e2: Vec<(u16, u16)> = [
      (2, 1),
      (3, 1),
      (4, 2),
      (1, 3),
      (2, 4),
      (3, 4),
    ].to_vec();

    assert!(c2.iter().all(|item| e2.contains(item)));
    
    game.evolve();
    game.evolve();
    game.evolve();
    game.evolve();
    game.evolve();
    let c3: Vec<(u16, u16)> = get_cell_vec_tuple(&game);
    assert!(c3.iter().all(|item| init_vec.contains(item)));
  }