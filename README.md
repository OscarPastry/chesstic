# Chesstic ♟️

A fully-featured Chess Engine and Graphical User Interface built in Rust using the [`ggez`](https://ggez.rs/) framework.

## ✨ Features

### Gameplay & FIDE Rules
- **Complete Move Validation:** Handles all complex chess rules including *En Passant*, *Castling*, and *Pawn Promotion*.
- **Comprehensive Draw Detection:**
  - Stalemate
  - 50-Move Rule
  - Threefold Repetition
  - Insufficient Material (K v K, K+N v K, K+B v K, K+B v K+B)
- **Checkmate Detection**

### Built-in Chess Engine
- **Minimax Algorithm:** Uses Alpha-Beta pruning for fast decision trees.
- **Quiescence Search:** Eliminates the "horizon effect" by fully resolving tactical capture sequences at the leaf nodes.
- **Piece-Square Tables:** Evaluates piece positioning to encourage central control, development, and king safety.
- **Real-time Evaluation:** Displays current positional evaluation (+/- advantage) and recommends the "Best Move" on the sidebar.
- **Highly Optimized:** Zero-allocation move generation during tree search and efficient ray-casting attack detection.

### User Interface
- **Interactive Board:** Click to select pieces and view valid moves (highlighted with dots).
- **Sidebar Controls:**
  - **Rotate Board:** Flip the perspective seamlessly between White and Black.
  - **Undo Move:** Made a mistake? Rewind the game history step-by-step.
  - **Reset Game:** Instantly clear the board and start a new match.
- **Visual Aids:** Highlights selected pieces, flashes the screen on invalid inputs, and displays promotion selection menus right on the board.

## 🚀 Getting Started

### Prerequisites
You will need [Rust and Cargo](https://rustup.rs/) installed on your machine.
Since `ggez` relies on system graphics APIs, make sure you have standard graphics drivers installed (usually built-in for Windows/macOS, requires `alsa` and `udev` headers on some Linux distros).

### Installation & Running

1. Clone the repository:
```bash
git clone <your-repo-url>
cd chess
```

2. Run the game:
> **Note:** It is *highly* recommended to run the game in release mode. The chess engine calculates thousands of positions per second, and Rust's debug mode is significantly slower.

```bash
cargo run --release
```

## 🎮 Controls

* **Left Click:** Select a piece, move to a highlighted square, or select a promotion piece.
* **Right Sidebar:** Click the UI buttons to Rotate, Undo, or Reset. (The window must be wide enough, > 800px, to see the sidebar).

## 🛠️ Technical Stack
* **Language:** Rust (2024 Edition)
* **Graphics/Windowing:** [ggez](https://crates.io/crates/ggez)
* **SVG Rendering:** [resvg](https://crates.io/crates/resvg) & [tiny-skia](https://crates.io/crates/tiny-skia) (used to load and scale vector piece graphics seamlessly).

## 📁 Project Structure
- `src/main.rs`: Contains the entire engine, logic, and rendering pipeline.
- `pieces/`: Contains the `.svg` images for the White and Black chess pieces.
