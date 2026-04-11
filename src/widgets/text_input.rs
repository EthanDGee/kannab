//! A customizable text input widget for the TUI.

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Style,
    text::{Line, Span},
    widgets::{Block, Paragraph, Widget},
};

use crate::{model::modal_state::CursorPosition, view::theme::ColorScheme};

/// A simple text input widget that can be single-line or multi-line.
pub struct TextInput<'a> {
    /// The block to wrap the widget in.
    block: Option<Block<'a>>,
    /// The text currently in the input.
    value: &'a str,
    /// The cursor position (char_index, line_index).
    cursor_pos: CursorPosition,
    /// Whether the input is active.
    active: bool,
    /// Whether to wrap long lines.
    wrap: bool,
    /// Whether the input supports multiple lines.
    is_multiline: bool,
    /// Color Palette.
    theme: ColorScheme,
    /// Ratatui styling.
    style: Style,
}

impl<'a> TextInput<'a> {
    /// Creates a new `TextInput` widget with the given theme, value, and cursor position.
    /// Defaults to single-line mode.
    pub fn new(theme: ColorScheme, value: &'a str, cursor_pos: CursorPosition) -> Self {
        let style = Style::default().fg(theme.body_text);
        Self {
            block: None,
            value,
            cursor_pos,
            active: false,
            wrap: false,
            is_multiline: false,
            theme,
            style,
        }
    }

    /// Sets the block (border, title, etc.) that wraps the text input.
    pub fn block(mut self, block: Block<'a>) -> Self {
        self.block = Some(block);
        self
    }

    /// Sets whether the input is currently active/focused.
    ///
    /// When active, the cursor will be rendered.
    pub fn active(mut self, active: bool) -> Self {
        self.active = active;
        self
    }

    /// Sets the base style for the text in the input.
    pub fn style(mut self, style: Style) -> Self {
        self.style = style;
        self
    }

    /// Sets whether long lines should wrap.
    pub fn wrap(mut self, wrap: bool) -> Self {
        self.wrap = wrap;
        self
    }

    /// Enables multi-line support for this input.
    pub fn multiline(mut self) -> Self {
        self.is_multiline = true;
        self.wrap = true;
        self
    }

    /// Renders the widget in single-line mode.
    fn render_single_line(&self) -> Vec<Line<'a>> {
        vec![self.render_line(0, self.value)]
    }

    /// Renders the widget in multi-line mode.
    fn render_multi_line(&self) -> Vec<Line<'a>> {
        self.value
            .split('\n')
            .enumerate()
            .map(|(i, line)| self.render_line(i, line))
            .collect()
    }

    /// Renders a single line of text, adding cursor styling if it's the current line.
    fn render_line(&self, line_index: usize, line_content: &'a str) -> Line<'a> {
        // In single-line mode, we always render the cursor if active.
        // In multi-line mode, we only render it if the line_index matches.
        let should_render_cursor =
            self.active && (!self.is_multiline || line_index == self.cursor_pos.line_index);

        if should_render_cursor {
            self.render_active_line(line_content)
        } else {
            Line::from(Span::styled(line_content, self.style))
        }
    }

    /// Renders the line containing the cursor, highlighting the character at the cursor position.
    fn render_active_line(&self, line: &'a str) -> Line<'a> {
        let cursor_style = self
            .style
            .bg(self.theme.highlight)
            .fg(self.theme.background);
        let mut it = line.char_indices();

        match it.nth(self.cursor_pos.char_index) {
            Some((start, _)) => {
                let end = it.next().map_or(line.len(), |(idx, _)| idx);
                Line::from(vec![
                    Span::styled(&line[..start], self.style),
                    Span::styled(&line[start..end], cursor_style),
                    Span::styled(&line[end..], self.style),
                ])
            }
            None => Line::from(vec![
                Span::styled(line, self.style),
                Span::styled(" ", cursor_style),
            ]),
        }
    }
}

impl<'a> Widget for TextInput<'a> {
    /// Renders the `TextInput` widget onto the buffer.
    fn render(self, area: Rect, buffer: &mut Buffer) {
        let text_area = self.block.as_ref().map_or(area, |block| {
            let inner = block.inner(area);
            block.render(area, buffer);
            inner
        });

        if text_area.height < 1 {
            return;
        }

        let lines = if self.is_multiline {
            self.render_multi_line()
        } else {
            self.render_single_line()
        };

        let mut paragraph = Paragraph::new(lines).style(self.style);
        if self.wrap {
            paragraph = paragraph.wrap(ratatui::widgets::Wrap { trim: false });
        }
        paragraph.render(text_area, buffer);
    }
}
