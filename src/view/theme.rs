//! Theme and color scheme definitions for the application.

use ratatui::style::Color;
use serde::{Deserialize, Serialize};

/// Defines the color palette used throughout the TUI.
#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct ColorScheme {
    /// Background color for the entire application.
    pub background: Color,
    /// Color for the outermost borders of the main views.
    pub outer_border: Color,
    /// Color for inner borders (e.g., within columns or modals).
    pub inner_border: Color,
    /// Color used for highlighting selected items or active fields.
    pub highlight: Color,
    /// Text color used on top of highlight backgrounds.
    pub highlight_text: Color,
    /// Standard color for general body text.
    pub body_text: Color,
}

impl Default for ColorScheme {
    /// Returns the default "Tokyo Night" inspired color scheme.
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
