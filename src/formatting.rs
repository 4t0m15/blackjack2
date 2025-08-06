/// Formatting utilities for creating aligned boxes and text displays
use std::fmt::Write;
use chrono::{DateTime, Local};

/// Box drawing characters for consistent formatting
pub struct BoxChars;

impl BoxChars {
    pub const TOP_LEFT: char = '╔';
    pub const TOP_RIGHT: char = '╗';
    pub const BOTTOM_LEFT: char = '╚';
    pub const BOTTOM_RIGHT: char = '╝';
    pub const HORIZONTAL: char = '═';
    pub const VERTICAL: char = '║';
    pub const T_DOWN: char = '╦';
    pub const T_UP: char = '╩';
    pub const T_RIGHT: char = '╠';
    pub const T_LEFT: char = '╣';
    pub const CROSS: char = '╬';
    pub const LIGHT_HORIZONTAL: char = '─';
    pub const LIGHT_VERTICAL: char = '│';
    pub const LIGHT_T_RIGHT: char = '├';
    pub const LIGHT_T_LEFT: char = '┤';
}

/// Creates a formatted box with title and content
pub struct BoxFormatter {
    width: usize,
    title: String,
    lines: Vec<String>,
}

impl BoxFormatter {
    /// Create a new box formatter with specified width
    #[must_use]
    pub fn new(width: usize, title: &str) -> Self {
        Self {
            width,
            title: title.to_string(),
            lines: Vec::new(),
        }
    }

    /// Add a line to the box content
    pub fn add_line(&mut self, content: &str) {
        self.lines.push(content.to_string());
    }

    /// Add a formatted line with label and value
    pub fn add_field(&mut self, label: &str, value: &dyn std::fmt::Display) {
        let content = format!("{label}: {value}");
        self.lines.push(content);
    }

    /// Add a formatted line with label and value, right-aligned
    pub fn add_field_aligned(&mut self, label: &str, value: &dyn std::fmt::Display) {
        let value_str = value.to_string();
        let content_width = self.width - 4; // 4 for borders and spaces (║ space content space ║)
        let separator = ": ";
        
        // Calculate available space for padding
        let total_text_len = label.len() + separator.len() + value_str.len();
        
        if total_text_len <= content_width {
            let padding_needed = content_width - total_text_len;
            let content = format!("{label}{separator}{}{value_str}", " ".repeat(padding_needed));
            self.lines.push(content);
        } else {
            // If too long, fall back to simple format
            let content = format!("{label}{separator}{value_str}");
            self.lines.push(content);
        }
    }

    /// Add an empty line for spacing
    pub fn add_empty_line(&mut self) {
        self.lines.push(String::new());
    }

    /// Add a separator line
    pub fn add_separator(&mut self) {
        let separator = format!(
            "{}{}{}",
            BoxChars::LIGHT_T_RIGHT,
            BoxChars::LIGHT_HORIZONTAL.to_string().repeat(self.width - 2),
            BoxChars::LIGHT_T_LEFT
        );
        self.lines.push(separator);
    }

    /// Generate the complete formatted box
    #[must_use]
    pub fn build(&self) -> String {
        let mut result = String::new();
        
        // Top border with title
        if self.title.is_empty() {
            // Simple top border without title
            writeln!(
                result,
                "{}{}{}",
                BoxChars::TOP_LEFT,
                BoxChars::HORIZONTAL.to_string().repeat(self.width - 2),
                BoxChars::TOP_RIGHT
            )
            .unwrap();
        } else {
            // Top border with centered title
            let title_with_spaces = format!(" {} ", self.title);
            let title_len = title_with_spaces.len();

            if title_len >= self.width - 2 {
                // Title too long, truncate it
                let max_title_len = self.width - 6; // Leave space for borders and spaces
                let truncated_title = if self.title.len() > max_title_len {
                    format!(" {}... ", &self.title[..max_title_len.saturating_sub(3)])
                } else {
                    title_with_spaces
                };
                writeln!(
                    result,
                    "{}{}{}",
                    BoxChars::TOP_LEFT,
                    truncated_title,
                    BoxChars::TOP_RIGHT
                )
                .unwrap();
            } else {
                // Center the title
                let remaining_space = self.width - 2 - title_len;
                let left_padding = remaining_space / 2;
                let right_padding = remaining_space - left_padding;

                writeln!(
                    result,
                    "{}{}{}{}{}",
                    BoxChars::TOP_LEFT,
                    BoxChars::HORIZONTAL.to_string().repeat(left_padding),
                    title_with_spaces,
                    BoxChars::HORIZONTAL.to_string().repeat(right_padding),
                    BoxChars::TOP_RIGHT
                )
                .unwrap();
            }
        }

        // Content lines
        for line in &self.lines {
            if line.contains(BoxChars::LIGHT_T_RIGHT) && line.contains(BoxChars::LIGHT_T_LEFT) {
                // This is a separator line, use it as-is
                writeln!(result, "{line}").unwrap();
            } else {
                let content_width = self.width - 4; // 2 for borders, 2 for spaces
                let truncated = if line.len() > content_width {
                    format!("{}...", &line[..content_width.saturating_sub(3)])
                } else {
                    line.clone()
                };
                let padding = content_width.saturating_sub(truncated.len());
                writeln!(
                    result,
                    "{} {}{} {}",
                    BoxChars::VERTICAL,
                    truncated,
                    " ".repeat(padding),
                    BoxChars::VERTICAL
                )
                .unwrap();
            }
        }

        // Bottom border
        write!(
            result,
            "{}{}{}",
            BoxChars::BOTTOM_LEFT,
            BoxChars::HORIZONTAL.to_string().repeat(self.width - 2),
            BoxChars::BOTTOM_RIGHT
        )
        .unwrap();

        result
    }
}

/// Format a percentage with proper alignment
#[must_use]
pub fn format_percentage(value: f64) -> String {
    format!("{value:.1}%")
}

/// Format money values with proper signs
#[must_use]
pub fn format_money(amount: i32) -> String {
    if amount >= 0 {
        format!("+{amount}")
    } else {
        amount.to_string()
    }
}

/// Format time duration
#[must_use]
pub fn format_duration(start: &DateTime<Local>) -> String {
    let now = Local::now();
    let duration = now.signed_duration_since(*start);
    
    if let Ok(std_duration) = duration.to_std() {
        let total_seconds = std_duration.as_secs();
        let hours = total_seconds / 3600;
        let minutes = (total_seconds % 3600) / 60;
        let seconds = total_seconds % 60;
        
        if hours > 0 {
            format!("{hours}h {minutes}m {seconds}s")
        } else if minutes > 0 {
            format!("{minutes}m {seconds}s")
        } else {
            format!("{seconds}s")
        }
    } else {
        "0s".to_string()
    }
}

/// Create a simple horizontal line
#[must_use]
pub fn create_line(width: usize, char: char) -> String {
    char.to_string().repeat(width)
}

/// Pad text to center it within a given width
#[must_use]
pub fn center_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        text.to_string()
    } else {
        let padding = (width - text.len()) / 2;
        let right_padding = width - text.len() - padding;
        format!(
            "{}{}{}",
            " ".repeat(padding),
            text,
            " ".repeat(right_padding)
        )
    }
}

/// Pad text to right-align it within a given width
#[must_use]
pub fn right_align_text(text: &str, width: usize) -> String {
    if text.len() >= width {
        text.to_string()
    } else {
        let padding = width - text.len();
        format!("{}{}", " ".repeat(padding), text)
    }
}
