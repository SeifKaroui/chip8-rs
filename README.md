# CHIP-8 interpreter
## Introduction 
This is a CHIP-8 interpreter written in rust as a personal project to get familiar with rust and learn how interpreters work.
## Resources
* [Guide to making a CHIP-8 emulator](https://tobiasvl.github.io/blog/write-a-chip-8-emulator/#fx1e-add-to-index)
* [Cowgod's Chip-8 Technical Reference](http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.5)
* [A Chip-8 Interpreter written in C](https://github.com/cookerlyk/Chip8)
* [Chip-8 VM built in rust](https://github.com/starrhorne/chip8-rust)
 
## Requirements

* Install sdl2. 
Ubuntu example:

```
sudo apt-get install libsdl2-dev libsdl2-gfx-dev
```

For more information visit [rust-sdl2 documentation](https://github.com/Rust-SDL2/rust-sdl2?tab=readme-ov-file#linux)

## How to run
```
$ ./chip8-interpreter roms\space-invaders.ch8
```
## Options:
```
$ ./chip8-interpreter --help
Chip8 interpreter written in rust

Usage: ./chip8-interpreter [OPTIONS]

Options:
  -f, --filename <FILENAME>  Path to rom file [default: roms\space-invaders.ch8]
  -a, --azerty               Default keyboard layout is QWERTY
  -h, --help                 Print help
  -V, --version              Print version
```

## Keybinds
'esc' Key  : Close the Emulator<br>
'P' : Pause / Resume the Emulator<br>

### Chip8 Keypad:
|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | C |
| 4 | 5 | 6 | D |
| 7 | 8 | 9 | E |
| A | 0 | B | F |

### Emulator Keyboard Mapping:
#### Qwerty layout (default)

|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| Q | W | E | R |
| A | S | D | F |
| Z | X | C | V |

#### Azerty layout

|   |   |   |   |
|---|---|---|---|
| 1 | 2 | 3 | 4 |
| A | Z | E | R |
| Q | S | D | F |
| W | X | C | V |





