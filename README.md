# snake-tui

A terminal-based Snake game implemented in Rust. This project features a simple and interactive user interface using a terminal TUI (Text User Interface), allowing you to play the classic Snake game directly in your terminal. 

You can install it via `cargo` from [crates.io](https://crates.io) or build it from source.

[![Crates.io](https://img.shields.io/crates/v/snake-tui.svg)](https://crates.io/crates/snake-tui)

## Features

- **Classic Snake Gameplay**: The game follows the traditional Snake mechanics.
- **Terminal-based User Interface (TUI)**: Built to be fully played within your terminal.
- **Cross-platform**: Compatible with any system that supports Rust and the terminal.

## Table of Contents

- [Installation](#installation)
- [Building from Source](#building-from-source)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)

## Installation

### From Crates.io

You can easily install the Snake Terminal UI directly using `cargo` by running the following command:

```bash
cargo install snake-tui
```

This will download and install the latest version of the game from crates.io.

## Building from source

To build the project from source, follow these steps:

1. Clone the repository
    ```bash
    git clone https://github.com/yourusername/snake-tui.git
    cd snake-tui
    ```
2. Build the project using cargo
    ```bash
    cargo build --release
    ```
3. Run the game
    ```bash
    cargo run
    ```
This will compile the code and start the game in your terminal.

## Usage

After installation or building from source, you can start the game by simply running:

```bash
snake-tui
```

Control the snake using the arrow keys and try to eat the food to grow the snake longer. The game ends when the snake collides with itself.
