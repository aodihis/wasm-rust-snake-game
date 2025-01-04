# Snake Game Using Rust and WebAssembly

A classic **Snake Game** built using **Rust** and **WebAssembly**, rendered in the browser using the HTML5 Canvas API.

---

## Features
- Classic snake gameplay: eat food to grow longer, avoid crashing into walls or yourself.
- Grid-based movement for smooth gameplay.
- HTML5 Canvas for rendering the game in the browser.
- Written in Rust, compiled to WebAssembly for high performance.

---

## Prerequisites

To build and run this project, ensure you have the following installed:

- [Rust](https://www.rust-lang.org/tools/install)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
- A simple HTTP server to run the html file.

---

## Getting Started

1. **Clone the Repository:**
   ```bash
   git clone <repository-url>
   cd snake_game
   ```

2. **Build the Project:**
   Compile the Rust code into WebAssembly using `wasm-pack`:
   ```bash
   wasm-pack build --target web
   ```

   This will generate a `pkg/` directory containing the compiled WebAssembly and JavaScript bindings.

3. **Serve the Project:**
   Serve the project using a simple HTTP server. 
   For starter, there is an index.html file in this project as an example.

   Open your browser and navigate it to your page to play the game.

---

## Folder Structure
```
snake_game/
├── pkg/                     # Compiled WebAssembly and JavaScript bindings (generated by wasm-pack)
├── src/                     # Source code for the Rust project
│   ├── lib.rs               # Main game logic written in Rust
├── index.html               # HTML file for the game as an example
├── Cargo.toml               # Rust project configuration file
├── Cargo.lock               # Auto-generated lock file for dependencies
└── README.md                # Project documentation
```

---

## How to Play
1. Use the **arrow keys** on your keyboard to control the snake's direction.
2. Collect the red food blocks to grow longer.
3. Avoid running into the walls or your own body.
4. The game ends when the snake crashes.

---

## Customization
You can tweak the game parameters to suit your preferences:
- **Grid size**: Modify `self.width` and `self.height` in the Rust code.
- **Cell size**: Adjust the multiplier (e.g., `10.0`) in `draw()` functions to increase or decrease the cell size.
- **Speed**: Adjust the game loop's update frequency.

---

Enjoy coding and playing your Snake Game! 🐍

