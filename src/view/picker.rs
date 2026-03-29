#[derive(Clone, Default)]
pub struct PickerState {
    pub index: usize,
}

impl PickerState {
    pub fn new() -> Self {
        Self::default()
    }
}
