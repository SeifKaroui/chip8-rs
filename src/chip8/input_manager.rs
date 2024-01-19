use sdl2::{event::Event, keyboard::Keycode, Sdl};
pub enum KeyboardType {
    Qwerty,
    Azerty,
}
pub enum GameState {
    Running([bool; 16]),
    Paused,
    Exited,
}
pub struct InputManager {
    event_pump: sdl2::EventPump,
    key_map_fn: fn(Keycode) -> Option<usize>,
    is_paused: bool,
}
impl InputManager {
    pub fn new(context: &Sdl, keyboard_type: KeyboardType) -> InputManager {
        InputManager {
            event_pump: context.event_pump().unwrap(),
            key_map_fn: match keyboard_type {
                KeyboardType::Azerty => key_map_azerty,
                _ => key_map_qwerty,
            },
            is_paused: false,
        }
    }
    pub fn process_input(&mut self) -> GameState {
        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => return GameState::Exited,
                Event::KeyDown {
                    keycode: Some(Keycode::P),
                    ..
                } => {
                    self.is_paused = !self.is_paused;
                }
                _ => {}
            }
        }
        if self.is_paused {
            return GameState::Paused;
        }

        let keys: Vec<Keycode> = self
            .event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(Keycode::from_scancode)
            .collect();

        let mut keypad = [false; 16];

        for key in keys {
            let result = (self.key_map_fn)(key);
            if let Some(idx) = result {
                keypad[idx] = true;
            }
        }
        return GameState::Running(keypad);
    }
}

fn key_map_qwerty(keycode: Keycode) -> Option<usize> {
    return match keycode {
        Keycode::X => Some(0),
        Keycode::Num1 => Some(1),
        Keycode::Num2 => Some(2),
        Keycode::Num3 => Some(3),
        Keycode::Q => Some(4),
        Keycode::W => Some(5),
        Keycode::E => Some(6),
        Keycode::A => Some(7),
        Keycode::S => Some(8),
        Keycode::D => Some(9),
        Keycode::Z => Some(10),
        Keycode::C => Some(11),
        Keycode::Num4 => Some(12),
        Keycode::R => Some(13),
        Keycode::F => Some(14),
        Keycode::V => Some(15),
        _ => None,
    };
}

fn key_map_azerty(keycode: Keycode) -> Option<usize> {
    return match keycode {
        Keycode::X => Some(0),
        Keycode::Num1 => Some(1),
        Keycode::Num2 => Some(2),
        Keycode::Num3 => Some(3),
        Keycode::A => Some(4),
        Keycode::E => Some(6),
        Keycode::Z => Some(5),
        Keycode::Q => Some(7),
        Keycode::S => Some(8),
        Keycode::D => Some(9),
        Keycode::W => Some(10),
        Keycode::C => Some(11),
        Keycode::Num4 => Some(12),
        Keycode::R => Some(13),
        Keycode::F => Some(14),
        Keycode::V => Some(15),
        _ => None,
    };
}
