# kanab

A keyboard-driven Kanban board manager for the terminal, built with Rust and Ratatui.

`kanab` is designed for developers and power users who want a fast, vim-inspired workflow for managing tasks without leaving the command line. It features a responsive TUI, global persistent storage, and intuitive keybindings for seamless board management.

## Features

- **Board Picker**: Manage multiple boards from a centralized launcher.
- **Customizable Columns**: Create, rename, delete, and reorder columns to fit your workflow.
- **Vim-Style Navigation**: Support for both arrow keys and standard `hjkl` navigation.
- **Global Persistence**: Boards are stored globally on your filesystem with automatic saving.
- **Responsive UI**: A fullscreen terminal interface that adapts to your window size with clear visual highlighting.

## Getting Started

### Installation

Since `kanab` is built with Rust, you can install it using `cargo`:

```bash
# Clone the repository
git clone https://github.com/yourusername/kanab.git
cd kanab

# Build and install
cargo install --path .
```

### Usage

Simply run the command to launch the board picker:

```bash
kanab
```

## Keybindings

### Board Picker (Main Menu)

| Key | Action |
|-----|--------|
| `↑` / `k` | Previous board |
| `↓` / `j` | Next board |
| `Enter` | Open selected board |
| `c` | Create new board |
| `r` / `e` | Rename board |
| `d` | Delete board |
| `?` | Show help overlay |
| `q` | Quit |

### Board View (Task Navigation)

| Key | Action |
|-----|--------|
| `←` / `h` | Previous column |
| `→` / `l` | Next column |
| `↑` / `k` | Previous task |
| `↓` / `j` | Next task |
| `Shift` + `↑` / `K` | Move task up |
| `Shift` + `↓` / `J` | Move task down |
| `Shift` + `←` / `H` | Move task to previous column |
| `Shift` + `→` / `L` | Move task to next column |
| `n` / `c` | New task |
| `e` | Edit task |
| `d` | Delete task |
| `C` | Create column |
| `E` | Edit column name |
| `D` | Delete column |
| `q` / `Esc` | Return to board picker |

## Storage

`kanab` stores your data in platform-specific directories:

- **Linux**: `~/.local/share/kanab/`
- **macOS**: `~/Library/Application Support/kanab/`
- **Windows**: `%APPDATA%/kanab/`

## Development

### Prerequisites

- Rust (latest stable version)
- Cargo

### Running Tests

```bash
cargo test
```

## License

This project is licensed under the Apache License, Version 2.0 - see the [LICENSE](LICENSE) file for details.
