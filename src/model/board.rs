use uuid::Uuid;

/// Handles the member of a todo list for a given task
///
/// id: Uuid
/// pub description: String
/// complete: bool
pub struct Item {
    id: Uuid,
    pub description: String,
    complete: bool,
}

impl Item {
    pub fn new() -> Self {
        Item {
            id: Uuid::new_v4(),
            description: String::new(),
            complete: false,
        }
    }
}

/// The member task of a Kanban column They contain the name of the task, a more detailed
/// description of the task, the ability to have an attached to-do list, and whether or not the task
/// is completed.
///
/// id: Uuid,
/// title: String,
/// description: String,
/// complete: bool,
/// checklist: Option<Vec<Item> = the items of of the optional to-do list
pub struct Task {
    id: Uuid,
    title: String,
    description: String,
    complete: bool,
    checklist: Option<Vec<Item>>,
}

impl Task {
    pub fn new() -> Self {
        Task {
            id: Uuid::new_v4(),
            title: String::new(),
            description: String::new(),
            complete: false,
            checklist: None,
        }
    }
}

///  A named column of the Kanban board containing a list of tasks as well
///
/// id: Uuid,
/// title: String,
/// tasks: Vec<Task>,
pub struct Column {
    id: Uuid,
    title: String,
    tasks: Vec<Task>,
}

impl Column {
    pub fn new() -> Self {
        Column {
            id: Uuid::new_v4(),
            title: String::new(),
            tasks: Vec::new(),
        }
    }
}

/// The model containing the data of the Kanban board containing columns of tasks as well as it's
/// own name.
///
/// id: Uuid,
/// title: String,
/// pub columns: Vec<Column>,
pub struct Board {
    id: Uuid,
    title: String,
    pub columns: Vec<Column>,
}

impl Board {
    pub fn new() -> Self {
        Board {
            id: Uuid::new_v4(),
            title: String::new(),
            columns: Vec::new(),
        }
    }
}
