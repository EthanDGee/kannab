# Kanban CLI Tool - Requirements Document

## 1. Project Overview

**Project Name:** kanban  
**Type:** Terminal User Interface (TUI) CLI Application  
**Core Functionality:** A keyboard-driven kanban board manager with vim-style navigation that stores boards globally on the filesystem.  
**Target Users:** Developers, power users, and productivity enthusiasts who prefer terminal-based workflows.

---

## 2. User Stories

### Board Management

| Story | Priority |
|-------|----------|
| Board picker on launch | Must |
| Create new boards | Must |
| List all boards in picker | Must |
| Open board from picker | Must |
| Delete boards from picker | Must |
| Rename boards from picker | Should |
| Reorder boards in picker | Should |

### Column Management

| Story | Priority |
|-------|----------|
| Create columns | Must |
| Rename columns | Must |
| Delete columns | Must |
| Reorder columns | Should |

### Task Management

| Story | Priority |
|-------|----------|
| Create tasks with titles | Must |
| Add descriptions to tasks | Must |
| Edit existing tasks | Must |
| Delete tasks | Must |
| Move tasks between columns | Must |

### Navigation

| Story | Priority |
|-------|----------|
| Navigate with arrow keys | Must |
| Navigate with `hjkl` | Should |
| Search with `/` | Could |

### Persistence

| Story | Priority |
|-------|----------|
| Global storage across sessions | Must |
| Auto-save | Must |

---

## 3. Minimum Viable Product (MVP)

### Features

1. **Board Picker** - Launch with board list, create/delete/rename/reorder boards
2. **Customizable Columns** - Add, rename, delete, reorder columns (no fixed columns)
3. **Task CRUD** - Title and description, move between columns
4. **Dual Keybindings** - Arrow keys (primary) + vim `hjkl` (alternative)
5. **Vim Modes** - Normal, Insert, Command modes
6. **Global  Storage** - Stored in OS specific
7. **Auto-save** - Changes saved automatically
8. **Help Overlay** - `?` key shows keybindings
9. **Fullscreen UI** - Responsive terminal layout with visual highlighting

### MVP Keybindings

#### Main Menu (Board Picker)

| Key | Action | Priority |
|-----|--------|----------|
| `↑` / `k` | Previous board | Must |
| `↓` / `j` | Next board | Must |
| `Home` / `g` | First board | Could |
| `End` / `G` | Last board | Could |
| `Shift+↑` / `K` | Move board up | Could |
| `Shift+↓` / `J` | Move board down | Could |
| `c` | Create new board | Must |
| `r` / `e` | Rename board | Should |
| `d` | Delete board | Must |
| `Enter` | Open board | Must |
| `?` | Help | Must |
| `q` | Quit | Must |

#### Board View (Column/Task Navigation)

| Key | Action | Priority |
|-----|--------|----------|
| `←` / `h` | Previous column | Must |
| `→` / `l` | Next column | Must |
| `↑` / `k` | Previous task | Must |
| `↓` / `j` | Next task | Must |
| `Home` / `g` | First task | Could |
| `End` / `G` | Last task | Could |
| `Shift+↑` / `K` | Move task up | Must |
| `Shift+↓` / `J` | Move task down | Must |
| `Shift+←` / `H` | Move task to previous column | Must |
| `Shift+→` / `L` | Move task to next column | Must |
| `Ctrl+←` / `Ctrl+h` | Move column left | Should |
| `Ctrl+→` / `Ctrl+l` | Move column right | Should |
| `n` / `c` | New task | Must |
| `e` | Edit task | Must |
| `d` | Delete task | Must |
| `C` | Create column | Must |
| `E` | Edit column name | Must |
| `D` | Delete column | Must |
| `/` | Search | Could |
| `?` | Help | Must |
| `q` / `esc` | Quit to board picker | Must |

#### Task Edit Mode

| Key | Action | Priority |
|-----|--------|----------|
| `←` / `→` | Move cursor left / right | Must |
| `↑` / `↓` | Move cursor up / down | Should |
| `Home` / `End` | Line start / end | Should |
| `Backspace` / `Delete` | Delete character | Must |
| `Enter` | New line | Must |
| `Tab` | Next field | Must |
| `Esc` | Close | Must |

### UI/UX Design

#### Layout Principles

- **Fullscreen terminal** - Use entire terminal window, responsive to resize
- **Horizontal column layout** - Columns arranged left-to-right, horizontally scrollable
- **Per-column scrolling** - Each column scrolls independently when tasks overflow
- **Task cards fill column width** - Cards expand to column width with text wrapping.

#### Visual Highlighting

| Element | Default | Selected |
|---------|---------|----------|
| Board (picker) | Normal text | **Bold** + `▶` prefix indicator |
| Column header | Normal | **Inverted colors** (bg/fg swap) |
| Task card | Normal + border | **Inverted colors** + bold text |
| Input field | Border only | **Inverted colors** + cursor |

#### Color Scheme (Terminal Attributes)

| Element | Style |
|---------|-------|
| Background | Terminal default |
| Selected item | Inverted) |
| Column borders | Dim/gray |
| Task count badge | Cyan text |
| Help overlay | Dark overlay + white text |
| Confirmation dialog | Yellow border + text |

#### Interaction Feedback

| Action | Visual Feedback |
|--------|-----------------|
| Select board/task | Immediate invert |
| Delete | Confirmation dialog appears |
| Save | Status message |

#### Responsive Behavior

| Terminal Width | Behavior |
|----------------|----------|
| Wide (>120 cols) | All columns visible |
| Medium (80-120 cols) | Horizontal scroll columns |
| Narrow (<80 cols) | Horizontal scroll, narrower columns |

| Terminal Height | Behavior |
|-----------------|---------|
| Normal | All columns same height |
| Short (<10 rows) | Compact mode, minimal padding |
| Per-column overflow | Each column scrolls vertically |

---

## 4. Extended Features (Post-MVP)

### Task Enrichment

- Checklists (sub-tasks within cards)
- Tags with colors
- Due dates with overdue indicators
- Priority levels (low/medium/high)
- Story Points
- Time tracking

### Navigation

- Jump to column by number (`1`, `2`, `3`)
- Jump to first/last task (`g`, `G`)
- Quick board switch (`1-9`)

### Data Interop

- export/import

### Configuration

- Color themes
- Custom keybindings
- Default board selection

---

## 5. Data Model

### Storage Structure

```
~/.local/share/kanban/
├── boards/
│   ├── work.json
│   └── personal.json
└── state.json
```

---

## 6. Acceptance Criteria

### MVP Must Have

- [ ] Board picker on launch
- [ ] Create/delete/rename boards works
- [ ] Customizable columns (add, rename, delete)
- [ ] Add tasks to any column
- [ ] Move tasks between columns (Shift+←/→ or H/L)
- [ ] Reorder tasks within column (Shift+↑/↓ or K/J)
- [ ] Edit task title and description
- [ ] Delete tasks
- [ ] Arrow key navigation works (primary)
- [ ] Vim `hjkl` navigation works (alternative)
- [ ] Normal/Insert/Command modes work
- [ ] Data persists after restart
- [ ] `?` shows help overlay
- [ ] Visual highlighting (invert on selection)
- [ ] Responsive fullscreen layout

### Should Have

- [ ] Reorder boards in picker
- [ ] Task count per column
- [ ] Empty state messages
- [ ] Confirmation for destructive actions

---

## 7. Non-Functional Requirements

| Requirement | Target |
|-------------|--------|
| Startup time | < 200ms |
| Cross-platform | Linux, macOS, Windows, freeBSD |

### Path Conventions

| Platform | Data Path |
|----------|-----------|
| Linux | `$XDG_DATA_HOME/kanban/` (default: `~/.local/share/kanban/`) |
| macOS | `~/Library/Application Support/kanban/` |
| Windows | `%APPDATA%/kanban/` |
