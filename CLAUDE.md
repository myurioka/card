# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

This is a Rust WebAssembly game project that compiles Rust code to WebAssembly and runs it in the browser. The project uses Vite for development and building, with wasm-pack for WebAssembly compilation.

## Project Structure

- `src/wasm/` - Contains the Rust WebAssembly crate
  - `src/lib.rs` - Main entry point that starts the game loop
  - `src/engine.rs` - Core game engine with rendering, input handling, and game loop
  - `src/game.rs` - Game state machine with different stages (Ready, Playing, GameOver, etc.)
  - `src/browser.rs` - Browser-specific utilities and WebAssembly bindings
  - `src/common.rs` - Shared constants and utilities
  - `src/game/` - Game-specific modules (messages, maze components)
- `js/` - Generated WebAssembly output (wasm.js, wasm_bg.wasm)
- `static/` - Static assets (CSS files)
- `index.html` - Main HTML file that loads the WebAssembly module

## Development Commands

### Build and Development
```bash
# Start development server
npm run dev

# Build WebAssembly only
npm run build-wasm

# Full build (WASM + Vite)
npm run build

# Preview built application
npm run preview
```

### Rust WebAssembly Build
The project uses wasm-pack to compile Rust to WebAssembly:
- Target: web
- Output directory: `js/`
- Crate type: cdylib

## Architecture Notes

### Game Engine Architecture
- Uses async/await with futures for the game loop
- State machine pattern for different game stages (Ready, Howto, Playing, GameOver, GameClear)
- Canvas-based rendering with HTML5 Canvas API
- Input handling for keyboard, mouse, and touch events

### WebAssembly Integration
- Entry point: `main()` function in lib.rs
- Browser integration through web-sys bindings
- Uses wasm-bindgen for JavaScript interop
- Game runs in a spawned local async task

### Dependencies
Key Rust dependencies:
- wasm-bindgen: WebAssembly bindings to JavaScript
- web-sys: Web API bindings
- futures: Async programming
- console_error_panic_hook: Better error reporting in browser

## Important Files
- `Cargo.toml`: Rust project configuration with WebAssembly-specific settings
- `package.json`: Node.js project with Vite build scripts
- `index.html`: Loads the WebAssembly module and initializes the game