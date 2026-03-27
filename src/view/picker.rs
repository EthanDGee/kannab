pub struct PickerState {
    index: usize,
}

// Information for the picker screen
impl PickerState {
    pub fn new() -> Self {
        PickerState { index: 0 }
    }
}
