use std::{io, time::Duration};

use ::conway::conway::Conway;

fn main() -> io::Result<()> {
    let mut game: Conway = Conway::new(
        30, 
        20,
        [(8, 7), (8, 8), (8, 9)].to_vec(), 
        "Blinker Pattern".to_string(),
        10,
        Duration::from_secs_f32(0.5)
    );
    game.run()?;

    Ok(())
}