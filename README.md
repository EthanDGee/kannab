# kannab

<!--toc:start-->
- [kannab](#kannab)
  - [Features](#features)
  - [Getting Started](#getting-started)
    - [Installation](#installation)
    - [Usage](#usage)
  - [Keybindings](#keybindings)
    - [Board Picker (Main Menu)](#board-picker-main-menu)
    - [Board View (Task Navigation)](#board-view-task-navigation)
    - [Task Modal (Create / Edit Task)](#task-modal-create--edit-task)
    - [Other Modals (Board / Column)](#other-modals-board--column--help)
    - [Confirmation Dialog](#confirmation-dialog)
  - [Storage](#storage)
  - [Configuration](#configuration)
  - [Development](#development)
  - [License](#license)
<!--toc:end-->

A keyboard-driven Kanban board manager for the terminal, built with Rust and Ratatui.

`kannab` is designed for developers and power users who want a fast, vim-inspired
workflow for managing tasks without leaving the command line. It features a
responsive TUI, global persistent storage, and intuitive keybindings for seamless
board management.

## Features

- **Board Picker**: Manage multiple boards from a centralized launcher.
- **Customizable Columns**: Create, rename, delete, and reorder columns.
- **Task Management**: Create, edit, delete, and reorder tasks with multi-line
descriptions and checklist items.
- **Checklist Items**: Add sub-items with completion toggling inside tasks.
- **Task Completion**: Toggle task completion status with a single key.
- **Column & Board Reordering**: Reorder columns with `Ctrl`+arrows and boards
with `Shift`+arrows.
- **Vim-Style Navigation**: Full `hjkl` and arrow key support throughout.
- **Global Persistence**: Boards stored globally on your file system with
automatic saving every 5 seconds.
- **Custom Themes**: Configurable color scheme via `~/.config/kannab/theme.toml`
with a Tokyo Night default.
- **Delete Confirmation**: All destructive actions prompt a confirmation dialog.
- **Responsive UI**: A fullscreen terminal interface with horizontal scrolling
when content exceeds the window size.
- **Comprehensive Help Window**: A comprehensive help window that covers all
functionality of kannab.

## Getting Started

### Installation

Since `kannab` is built with Rust, you can install it using `cargo`:

```bash
cargo install kannab
```

Or clone and build from source:

```bash
# Clone the repository
git clone https://github.com/EthanDGee/kannab.git
cd kannab

# Build and install
cargo install --path .
```

### Usage

Simply run the command to launch the board picker:

```bash
kannab
```

## Keybindings

### Board Picker (Main Menu)

| Key | Action |
| ----- | -------- |
| `↑` / `k` / `BackTab` | Previous board |
| `↓` / `j` / `Tab` | Next board |
| `Shift`+`↑` / `K` | Move board up |
| `Shift`+`↓` / `J` | Move board down |
| `Enter` | Open selected board |
| `c` / `n` | Create new board |
| `r` / `e` | Rename board |
| `d` | Delete board (with confirmation) |
| `q` / `Esc` | Quit |

### Board View (Task Navigation)

| Key | Action |
| ----- | -------- |
| `←` / `h` | Previous column |
| `→` / `l` | Next column |
| `↑` / `k` | Previous task |
| `↓` / `j` | Next task |
| `Tab` | Toggle task completion |
| `Shift`+`↑` / `K` | Move task up |
| `Shift`+`↓` / `J` | Move task down |
| `Shift`+`←` / `H` | Move task to previous column |
| `Shift`+`→` / `L` | Move task to next column |
| `n` / `c` | New task |
| `e` | Edit task |
| `d` | Delete task (with confirmation) |
| `Shift`+`C` / `Ctrl`+`n` | Create column |
| `r` / `Shift`+`E` / `Ctrl`+`r` | Rename column |
| `Shift`+`D` / `Ctrl`+`d` | Delete column (with confirmation) |
| `Ctrl`+`←` / `h` | Move column left |
| `Ctrl`+`→` / `l` | Move column right |
| `q` / `Esc` | Return to board picker |

### Task Modal (Create / Edit Task)

The task modal has three input fields cycled via `Tab` / `Shift`+`Tab`:
**Title** → **Description** → **Checklist Items**.

| Key | Action |
| ----- | -------- |
| `Esc` | Close modal (discard changes) |
| `Tab` | Next input field |
| `Shift`+`Tab` / `BackTab` | Previous input field |
| `Ctrl`+`S` / `Ctrl`+`Enter` | Confirm / Save |
| `Enter` (on **Title**) | Confirm task |
| `Enter` (on **Description**) | Insert newline |
| `Enter` (on a checklist item) | Toggle item completion |
| `Ctrl`+`Backspace` / `Ctrl`+`H` | Delete current checklist item |

New checklist items are created automatically when you type in the last
empty item slot and navigate away.

### Other Modals (Board / Column / Help)

| Key | Action |
| ----- | -------- |
| `?` / `shift + /` | Opens the help window` |
| `Esc` | Close modal (discard changes) |
| `Enter` | Confirm |

### Confirmation Dialog

| Key | Action |
| ----- | -------- |
| `Enter` | Confirm deletion |
| `Esc` | Cancel |

## Storage

`kannab` stores your data in platform-specific directories:

- **Linux**: `~/.local/share/kannab/`
- **macOS**: `~/Library/Application Support/kannab/`
- **Windows**: `%APPDATA%/kannab/`

## Configuration

The color theme is configured via `~/.config/kannab/theme.toml`. A default
theme is created automatically on first launch.

```toml
background = "#1A1B26"
outer_border = "#434C5E"
inner_border = "#434C5E"
highlight = "#7AA2F7"
highlight_text = "#CDD6F4"
body_text = "#CDD6F4"
```

To see theme changes you must close and restart the program. If the config
file is invalid or unreadable, the program falls back to the default Tokyo
Night theme.

## Development

See [CONTRIBUTING.md](docs/CONTRIBUTING.md) for full setup instructions,
commit guidelines, and pull request workflow.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE)
file for details.
