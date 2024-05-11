use std::io;

use ::conway::conway::Conway;

fn main() -> io::Result<()> {
    let mut game: Conway = Conway::default();
    game.run()?;

    Ok(())
}