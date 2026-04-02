use ratatui::style::Color;

#[derive(Clone, Copy)]
pub struct ColorScheme {
    pub background: Color,
    pub outer_border: Color,
    pub inner_border: Color,
    pub highlight: Color,
    pub highlight_text: Color,
    pub body_text: Color,
}

impl Default for ColorScheme {
    fn default() -> Self {
        ColorScheme {
            background: Color::Rgb(26, 27, 38),   // tokyo night background
            outer_border: Color::Rgb(67, 76, 94), // tokyo night border
            inner_border: Color::Rgb(67, 76, 94), // same as outer
            highlight: Color::Rgb(122, 162, 247), // tokyo night blue
            highlight_text: Color::Rgb(205, 214, 244), // tokyo night fg
            body_text: Color::Rgb(205, 214, 244), // tokyo night fg
        }
    }
}
