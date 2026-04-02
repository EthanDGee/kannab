//! Definitions for all available application actions and input fields.

use crate::model::modal_state::ModalType;

/// All possible actions that can trigger a state update.
/// This enum is the core of the Elm-like update loop.
pub enum Action {
    // Navigation
    /// Move selection up in the current view.
    MoveUp,
    /// Move selection down in the current view.
    MoveDown,
    /// Move selection to the left in the current view.
    MoveLeft,
    /// Move selection to the right in the current view.
    MoveRight,

    // Board Picker Actions
    /// Open the currently selected board.
    OpenBoard,
    /// Create a new board with the given title.
    CreateBoard(String),
    /// Rename the currently selected board.
    RenameBoard(String),
    /// Delete the currently selected board.
    DeleteBoard,
    /// Move the currently selected board up in the board list.
    MoveBoardUp,
    /// Move the currently selected board down in the board list.
    MoveBoardDown,
    /// Close the active board and return to the picker.
    QuitToPicker,

    // Board View Actions
    /// Select the next column to the right.
    NextColumn,
    /// Select the previous column to the left.
    PrevColumn,
    /// Select the next task (below) in the column.
    NextTask,
    /// Select the next task (above) in the column.
    PrevTask,

    // Column Actions
    /// Create a new column with the given title.
    CreateColumn(String),
    /// Rename the currently selected column.
    RenameColumn(String),
    /// Delete the currently selected column.
    DeleteColumn,
    /// Swap the currently selected column with the one to its left.
    MoveColumnLeft,
    /// Swap the currently selected column with the one to its right.
    MoveColumnRight,

    // Task Actions
    /// Create a new task in the currently selected column.
    CreateTask(String, String),
    /// Update a specific field of the currently selected task.
    EditTask(InputField, String),
    /// Delete the currently selected task.
    DeleteTask,
    /// Toggle the completion status of the currently selected task.
    ToggleCompletion,
    /// Move the current task up within its column.
    MoveTaskUp,
    /// Move the current task down within its column.
    MoveTaskDown,
    /// Move the current task to the top of next (right) column.
    MoveTaskToNextColumn,
    /// Move the current task to the bottom of previous (left) column.
    MoveTaskToPrevColumn,

    // Modal Actions
    /// Show a specific modal dialog.
    OpenModal(ModalType),
    /// Close any currently open modal and discard its modal state.
    CloseModal,
    /// Update the text value of a field in an active modal.
    UpdateField(InputField, String),
    /// Update the cursor position within a modal field.
    MoveCursor(usize, usize),
    /// Confirm the current modal operation (e.g., Save).
    Confirm,
    /// Discard the current modal operation.
    Cancel,

    // IO operations
    /// Mark the state as containing unsaved changes.
    MarkDirty,
    /// Immediately save all app data to local application storage.
    Save,

    // System
    /// Periodic system event for background tasks (like auto-save).
    Tick,
    /// Initiate application shutdown.
    Quit,
}

/// Identifiers for all text input fields across various modals and views.
pub enum InputField {
    /// Field for entering a board title.
    BoardName,
    /// Field for entering a column name.
    ColumnName,
    /// Field for entering a task title.
    TaskTitle,
    /// Field for entering a task description.
    TaskDescription,
    /// Field for entering a checklist item title.
    TaskItem,
}
