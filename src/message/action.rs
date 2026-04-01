use crate::model::modal_state::ModalType;

/// Handle message actions as part of Elm Architecture
pub enum Action {
    // Navigation
    MoveUp,
    MoveDown,
    MoveLeft,
    MoveRight,
    //  MoveToStart,  Post-MVP
    //  MoveToEnd,  Post-MVP

    // Board Picker Actions
    OpenBoard,
    CreateBoard(String),
    RenameBoard(String),
    DeleteBoard,
    MoveBoardUp,
    MoveBoardDown,
    QuitToPicker,

    // Board View Actions
    NextColumn,
    PrevColumn,
    NextTask,
    PrevTask,

    // Column Actions
    CreateColumn(String),
    RenameColumn(String),
    DeleteColumn,
    MoveColumnLeft,
    MoveColumnRight,

    // Task Actions
    CreateTask(String, String),
    EditTask(InputField, String),
    DeleteTask,
    ToggleCompletion,
    MoveTaskUp,
    MoveTaskDown,
    MoveTaskToNextColumn,
    MoveTaskToPrevColumn,

    // Modal Actions
    OpenModal(ModalType),
    CloseModal,
    UpdateField(InputField, String),
    Confirm,
    Cancel,

    // IO operations
    MarkDirty,
    Save,

    // System
    Render,
    Quit,
}

/// All possible text input fields within the app
pub enum InputField {
    BoardName,
    ColumnName,
    TaskTitle,
    TaskDescription,
    TaskItem,
}
