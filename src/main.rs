extern crate sdl2;

use chip8::audio_manager::AudioManager;
use chip8::display_manager::DisplayManager;
use chip8::input_manager::{GameState, InputManager};
use chip8::rom_loader::RomLoader;
use chip8::Chip8;
use std::time::Duration;

mod chip8;
mod cli;

const FREQUENCY: usize = 600; //~600HZ
const DELAY_SOUND_FREQUENCY: usize = 60; //60HZ
const INSTRUCTIONS_PER_UPDATE: usize = FREQUENCY / DELAY_SOUND_FREQUENCY;

fn main() {
    let args = cli::get_args();
    let mut chip8 = Chip8::new();
    let rom_loader = RomLoader::new(args.filename.as_str());
    chip8.load(&rom_loader.rom);

    let sdl_context = sdl2::init().unwrap();
    let mut input_manager = InputManager::new(&sdl_context, args.keyboard_type);
    let audio_manager = AudioManager::new(&sdl_context);
    let mut display_manager = DisplayManager::new(&sdl_context);

    let mut cycles = 0;

    'main: loop {
        let mut state;
        loop {
            state = input_manager.process_input();
            match state {
                GameState::Running(_) => break,
                GameState::Paused => {}
                GameState::Exited => break 'main,
            }
        }
        if let GameState::Running(keypad) = state {
            cycles += 1;

            let result = chip8.execute_instruction(keypad);
            if let Some(screen) = result {
                display_manager.draw_screen(screen);
            }

            if cycles == INSTRUCTIONS_PER_UPDATE {
                chip8.update_timers();
                cycles = 0;
            }

            if chip8.should_beep() {
                audio_manager.start_beep();
            } else {
                audio_manager.stop_beep();
            }

            std::thread::sleep(Duration::from_millis((1000 / FREQUENCY) as u64));
        }
    }
}
