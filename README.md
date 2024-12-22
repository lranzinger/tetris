# Blocks

A modern webassembly implementation of the classic Tetris game written in Rust using Macroquad, supporting both desktop and mobile browsers.

## Features

- Classic Tetris gameplay mechanics
- Responsive design that adapts to window size
- Touch controls for mobile devices
- Keyboard controls for desktop
- Progressive level system
- High score tracking with browser storage
- Visual effects for line clears
- Debug mode with FPS counter

## Controls

### Touch Controls

- Swipe left/right: Move piece
- Long swipe: Fast movement
- Tip once: Rotate piece
- Hold: Drop piece

### Keyboard Controls

- Arrow Keys/WASD: Move and rotate
- Down/S: Drop piece
- Up/W: Rotate piece

## Build Instructions

```bash
# Debug build
./build.sh debug

# Release build
./build.sh release
```

## Play Online

Visit [https://play.ranzinger.dev](https://play.ranzinger.dev) to play directly in your browser.
