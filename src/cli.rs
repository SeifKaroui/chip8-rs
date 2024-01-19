use crate::chip8::input_manager::KeyboardType;
use clap::Parser;

/// Chip8 interpreter written in rust
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Path to rom file
    #[arg(short, long, default_value_t = String::from(r"roms\space-invaders.ch8"))]
    pub filename: String,

    /// Default KeyboardType is QWERTY
    #[arg(short, long, default_value_t = false)]
    pub azerty: bool,
}

pub struct Args {
    pub filename: String,
    pub keyboard_type: KeyboardType,
}

pub fn get_args() -> Args {
    let cli = Cli::parse();
    let keyboard_type = if cli.azerty {
        KeyboardType::Azerty
    } else {
        KeyboardType::Qwerty
    };

    return Args {
        filename: cli.filename,
        keyboard_type,
    };
}
