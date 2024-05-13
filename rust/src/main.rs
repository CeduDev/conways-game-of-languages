use std::process;
use std::{io, time::Duration};

use ::conways_game_of_languages::conway::Conway;

use ::conways_game_of_languages::pattern::{PATTERNS, Pattern, Patterns};

use clap::Parser;

/// Conway's Game of Life in Rust. 
/// The following patterns are accepted as an argument: Blinker, Toad, Beacon, Pulsar, Penta Decathlon,
/// Glider, Glider Gun, Bunnies
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the pattern to use
    #[arg(short, long, required = true)]
    pattern: String,

    /// Width of the screen
    #[arg(short, long, default_value_t = 40)]
    width: u16,

    /// Height of the screen
    #[arg(long, default_value_t = 40)]
    height: u16,

    /// How many generations to run the game
    #[arg(short, long, default_value_t = 10)]
    generations: u16,

    /// Speed in seconds of each generation
    #[arg(short, long, default_value_t = 1.0)]
    speed: f32
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let speed = args.speed as f32;

    let pattern_string: String = args.pattern;
    if !PATTERNS.contains(&pattern_string.as_str()) {
        eprintln!("Invalid pattern {} passed! Please use the help flag -h to see the available patterns.", pattern_string);
        process::exit(1);
    }

    let data_string: &str = include_str!("../data/Blinker.ron");
    let patterns_data: Patterns = ron::from_str(data_string).unwrap();
    
    let pattern: &Pattern = patterns_data.patterns.iter().find(|p| p.name == pattern_string).unwrap();

    let (max_x, max_y) = pattern.get_max_alive();

    if max_x + 1 > args.width {
        eprintln!("Given width of {} is too small, since the pattern needs at least {} of width to work properly", args.width, max_x + 1);
        process::exit(1);
    }

    if max_y + 1 > args.height {
        eprintln!("Given height of {} is too small, since the pattern needs at least {} of height to work properly", args.height, max_y + 1);
        process::exit(1);
    }

    let mut game: Conway = Conway::new(
        args.width, 
        args.height,
        pattern.alive_cells.clone(), 
        pattern.name.clone(),
        args.generations,
        Duration::from_secs_f32(speed)
    );
    game.run()?;

    Ok(())
}