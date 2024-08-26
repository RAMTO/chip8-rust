# Basic Chip-8 Emulator on Rust

CHIP-8 is a simple, interpreted programming language primarily used to write video games. It was developed in the 1970s for use on the COSMAC VIP and Telmac 1800 microcomputers. CHIP-8 gained popularity because it provided a straightforward way for programmers to create small games and programs that could run on various low-power devices.

## How to generate ROM:

https://github.com/RAMTO/chip8-rom-generator

## Current ROM

`example.ch8`

```
0x00, 0xE0, // Clear screen
0xA2, 0x20, // LD I, 0x220
0xA2, 0x30, // LD I, 0x230
0x12, 0x0A, // Jump to 0x210
0xA2, 0x40, // LD I, 0x240 (this will be skipped)
0xA2, 0x34, // LD I, 0x250
0x00, 0xDF, // Exit
```

## How to run

```
cargo run
```

## How to run tests

```
cargo test
```

## How to run with Docker

```
docker build -t chip8-rust .
```

and

```
docker run --rm chip8-rust
```
