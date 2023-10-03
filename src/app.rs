use ratatui::backend::Backend;
use ratatui::layout::Alignment;
use ratatui::layout::{Constraint, Direction, Layout};
use ratatui::style::{Color, Style};
use ratatui::terminal::Frame;
use ratatui::widgets::{Block, Borders, Paragraph};
use regex::Regex;
use std::error;

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

#[derive(Debug, PartialEq, Eq)]
pub enum CurrentField {
    Regex,
    Text,
    Matches,
}

impl CurrentField {
    pub fn next(&mut self) {
        match self {
            Self::Regex => *self = Self::Text,
            Self::Text => *self = Self::Matches,
            Self::Matches => *self = Self::Regex,
        }
    }

    pub fn is_text(&self) -> bool {
        self == &CurrentField::Text
    }

    pub fn is_regex(&self) -> bool {
        self == &CurrentField::Regex
    }

    pub fn is_matches(&self) -> bool {
        self == &CurrentField::Matches
    }
}

#[derive(Debug, PartialEq, Eq, Default)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub running: bool,
    pub current_input: CurrentField,
    pub re: Option<Regex>,
    pub regex: String,
    pub text: Vec<String>,
    pub matches: Vec<String>,
    pub regex_cursor_pos: Position,
    pub text_cursor_pos: Position,
    pub matches_cursor_pos: Position,
}

impl Default for App {
    fn default() -> Self {
        Self {
            running: true,
            current_input: CurrentField::Text,
            re: None,
            regex: String::new(),
            text: vec![String::new()],
            matches: vec![String::new()],
            regex_cursor_pos: Position::default(),
            text_cursor_pos: Position::default(),
            matches_cursor_pos: Position::default(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Renders the user interface widgets.
    pub fn render<B: Backend>(&mut self, frame: &mut Frame<'_, B>) {
        // This is where you add new widgets.
        // See the following resources:
        // - https://docs.rs/tui/0.16.0/tui/widgets/index.html
        // - https://github.com/fdehau/tui-rs/tree/v0.16.0/examples
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Percentage(50),
                    Constraint::Percentage(50),
                ]
                .as_ref(),
            )
            .split(frame.size());

        frame.render_widget(
            Paragraph::new(&self.regex[..])
                .block(Block::default().title("regex").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left),
            chunks[0],
        );

        frame.render_widget(
            Paragraph::new(&self.text.join("\n")[..])
                .block(Block::default().title("text").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left),
            chunks[1],
        );

        frame.render_widget(
            Paragraph::new(&self.matches.join("\n")[..])
                .block(Block::default().title("matches").borders(Borders::ALL))
                .style(Style::default().fg(Color::White).bg(Color::Black))
                .alignment(Alignment::Left),
            chunks[2],
        );

        match self.current_input {
            CurrentField::Regex => {
                frame.set_cursor(chunks[0].x + self.regex_cursor_pos.x + 1, chunks[0].y + 1);
            }
            CurrentField::Text => {
                frame.set_cursor(
                    chunks[1].x + self.text_cursor_pos.x + 1,
                    chunks[1].y + self.text_cursor_pos.y + 1,
                );
            }
            CurrentField::Matches => {
                frame.set_cursor(
                    chunks[2].x + self.matches_cursor_pos.x + 1,
                    chunks[2].y + self.matches_cursor_pos.y + 1,
                );
            }
        };
    }
}
