//! View components for rendering a full Kanban board.

use crate::APP_NAME;
use crate::app::App;
use crate::model::board_state::Board;
use crate::model::modal_state::ModalState;
use crate::view::column_view::{self, COLUMN_WIDTH};
use crate::widgets::floating_window::centered_rect;
use ratatui::Frame;
use ratatui::layout::{Constraint, Direction, Layout, Rect};
use ratatui::style::Modifier;
use ratatui::style::Style;
use ratatui::widgets::Paragraph;
use ratatui::widgets::{Block, Borders, Clear};

/// Handles associated view data for the Kanban board data.
///
/// In addition to the cursor position for tasks and current column, all scroll data for every column is stored
/// independently to ensure the scroll position is preserved when switching between columns.
pub struct BoardState {
    /// Index of the currently focused column.
    pub column_index: usize,
    /// Index of the currently focused task within the active column.
    pub task_index: usize,
    /// Persistent scroll positions for each column.
    pub column_scrolls: Vec<usize>,
    /// The underlying board data model.
    pub board: Board,
}

impl BoardState {
    /// Creates a new `BoardState` for the given board, initializing column scrolls to zero.
    pub fn new(board: Board) -> Self {
        let num_columns = board.columns.len();
        BoardState {
            column_index: 0,
            task_index: 0,
            column_scrolls: vec![0; num_columns],
            board,
        }
    }

    /// Returns true if the board contains no columns.
    pub fn column_list_empty(&self) -> bool {
        self.board.column_list_empty()
    }

    /// Returns true if the column at the provided index contains no tasks.
    pub fn task_list_empty(&self, column_index: usize) -> bool {
        self.board.task_list_empty(column_index)
    }

    /// Returns an immutable reference to the currently selected column.
    pub fn current_column(&self) -> Option<&crate::model::board_state::Column> {
        self.board.get_column(self.column_index)
    }

    /// Returns an immutable reference to the currently selected task.
    pub fn current_task(&self) -> Option<&crate::model::board_state::Task> {
        self.current_column()?.get_task(self.task_index)
    }

    /// Switches the currently focused column to the given index, preserving scroll positions.
    pub fn switch_column(&mut self, new_index: usize) {
        if new_index < self.board.columns.len() {
            // Save current task_index
            if self.column_index < self.column_scrolls.len() {
                self.column_scrolls[self.column_index] = self.task_index;
            }

            self.column_index = new_index;

            // Restore new task_index
            if self.column_index < self.column_scrolls.len() {
                self.task_index = self.column_scrolls[self.column_index];
            }

            // Clamp task_index to new column's task count
            let num_tasks = self.current_column().map_or(0, |c| c.tasks.len());
            if num_tasks == 0 {
                self.task_index = 0;
            } else if self.task_index >= num_tasks {
                self.task_index = num_tasks - 1;
            }
        }
    }
}

/// Renders the active board view, including its header, columns, and footer.
pub fn render(app: &App, frame: &mut Frame, area: Rect) {
    let board_state = app.model.board_state.as_ref().unwrap();
    let board = &board_state.board;

    let colors = &app.model.color_scheme;
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Kanban Manager
            Constraint::Length(1), // Footer
        ])
        .split(area);

    // Header
    let header_title = format!("{} - {}", APP_NAME, board.title);
    let title = Paragraph::new(header_title)
        .style(Style::default().fg(colors.body_text).bg(colors.background));
    frame.render_widget(title, chunks[0]);

    // Horizontal layout for columns with fixed width and scrolling
    let available_width = chunks[1].width;
    let max_visible_columns = (available_width / COLUMN_WIDTH).max(1) as usize;

    // Calculate which columns to display to keep the selected one in view
    // This is a simple "follow the cursor" scrolling logic
    let start_col = if board_state.column_index >= max_visible_columns {
        board_state
            .column_index
            .saturating_sub(max_visible_columns - 1)
    } else {
        0
    };

    let visible_count = std::cmp::min(board.columns.len() - start_col, max_visible_columns);

    let column_constraints: Vec<Constraint> =
        std::iter::repeat_n(Constraint::Length(COLUMN_WIDTH), visible_count).collect();

    let column_areas = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(column_constraints)
        .split(chunks[1]);

    for (i, column_area) in column_areas.iter().enumerate() {
        let actual_idx = start_col + i;
        if let Some(column) = board.columns.get(actual_idx) {
            let selected = actual_idx == board_state.column_index;
            column_view::render(app, frame, column, *column_area, selected, actual_idx);
        }
    }

    // Footer
    let footer_text = format!("{}  [?] Help", board.title);
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(colors.body_text).bg(colors.background));
    frame.render_widget(footer, chunks[2]);
}

/// Renders a modal for creating or renaming a board.
pub fn board_modal_view(
    app: &App,
    frame: &mut Frame,
    modal: &ModalState,
    area: Rect,
    title: &str,
    instruction_text: &str,
) {
    let colors = app.model.color_scheme;
    let area = centered_rect(60, 15, area);

    frame.render_widget(Clear, area); //this clears out the background

    let block = Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(colors.highlight))
        .style(Style::default().bg(colors.background).fg(colors.body_text));

    let inner_area = block.inner(area);
    frame.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Title Label
            Constraint::Length(3), // Input Field
            Constraint::Length(1), // Instructions
        ])
        .split(inner_area);

    let label = Paragraph::new("Board Name:").style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(label, chunks[0]);

    // Use TextArea for input
    let mut textarea = modal.active_textarea.clone();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(colors.inner_border)),
    );
    textarea.set_cursor_style(Style::default().fg(colors.background).bg(colors.highlight));
    frame.render_widget(&textarea, chunks[1]);

    let instructions =
        Paragraph::new(instruction_text).style(Style::default().fg(colors.inner_border));
    frame.render_widget(instructions, chunks[2]);
}

#[cfg(test)]
mod tests {
    use crate::model::board_state::{Board, Column, Task};
    use crate::view::board_view::*;

    #[test]
    fn test_board_state_new() {
        let mut board = Board::new("T".to_string());
        board.columns.push(Column::new());
        let bs = BoardState::new(board);
        assert_eq!(bs.column_index, 0);
        assert_eq!(bs.task_index, 0);
        assert_eq!(bs.column_scrolls.len(), 1);
    }

    #[test]
    fn test_switch_column_preserves_scroll() {
        let mut board = Board::new("T".to_string());

        let mut c1 = Column::new();
        c1.tasks.push(Task::new());
        c1.tasks.push(Task::new());

        let mut c2 = Column::new();
        c2.tasks.push(Task::new());

        board.columns.push(c1);
        board.columns.push(c2);

        let mut bs = BoardState::new(board);

        // At C1, move to Task index 1
        bs.task_index = 1;

        // Switch to C2
        bs.switch_column(1);
        assert_eq!(bs.column_index, 1);
        assert_eq!(bs.task_index, 0); // Restored from scroll (0)
        assert_eq!(bs.column_scrolls[0], 1); // Saved from C1

        // Switch back to C1
        bs.switch_column(0);
        assert_eq!(bs.column_index, 0);
        assert_eq!(bs.task_index, 1); // Restored from saved scroll
    }

    #[test]
    fn test_switch_column_clamping() {
        let mut board = Board::new("T".to_string());
        board.columns.push(Column::new()); // Empty
        board.columns.push(Column::new()); // Empty

        let mut bs = BoardState::new(board);
        bs.column_scrolls[1] = 99; // Manually set a high scroll

        bs.switch_column(1);
        assert_eq!(bs.task_index, 0); // Clamped to 0 since column is empty
    }
}
