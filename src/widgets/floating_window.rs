use ratatui::layout::Rect;

/// Creates the same centered rectangle but in addition to it's percentage it also intakes a flat
/// unit of the minimum size that the window.
pub fn centered_rect_minimum_size(
    percent_x: u16,
    percent_y: u16,
    min_width: u16,
    min_height: u16,
    rectangle: Rect,
) -> Rect {
    let mut width = rectangle.width * percent_x / 100;
    let mut height = rectangle.height * percent_y / 100;

    if min_width > 0 {
        width = width.max(min_width).min(rectangle.width);
    }
    if min_height > 0 {
        height = height.max(min_height).min(rectangle.height);
    }

    let x = rectangle.x + (rectangle.width.saturating_sub(width)) / 2;
    let y = rectangle.y + (rectangle.height.saturating_sub(height)) / 2;

    Rect {
        x,
        y,
        width,
        height,
    }
}

/// Helper function to create a centered rectangle using up to a certain percentage of the available area.
#[allow(dead_code)]
pub fn centered_rect(percent_x: u16, percent_y: u16, rectangle: Rect) -> Rect {
    centered_rect_minimum_size(percent_x, percent_y, 0, 0, rectangle)
}
