// ---------------------
// Navigation Utilities
// ---------------------

pub fn increment_no_wrap(index: usize, max: usize) -> Option<usize> {
    if (index + 1) > max {
        None
    } else {
        Some(index + 1)
    }
}

pub fn decrement_no_wrap(index: usize) -> Option<usize> {
    if index == 0 { None } else { Some(index - 1) }
}
