use std::time::{Duration, Instant};

use cli_clipboard::{ClipboardContext, ClipboardProvider};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers};
use futures::io;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style, Stylize},
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph, Widget},
    Frame,
};
use tui_textarea::{Input, TextArea};
use wake_core::agent::{AgentController, AgentEvent, PublicAgentState};
use wake_llm::{tool::call_fc_auto::ToolCallFunctionCallingAuto, ToolCallMethod};

use crate::tui::{cmdnav::CommandNav, helper::HelpArea};

use super::theme::WAKE_YELLOW;

pub enum UserAction {
    Nope,
    CancelTask,
    UserInput { input: String },
    UserAppCommand { command: String },
}

pub struct InputArea<'a> {
    agent_running: bool,

    // input text
    input: TextArea<'a>,
    placeholder: String,

    // draft saving for history navigation
    current_draft: Option<String>,

    // alert top left
    animation_start: Option<Instant>,
    status_message: Option<String>,

    // status bottom left
    last_keystroke_time: Option<Instant>,
    pending_enter: Option<Instant>,
    helper_msg: Option<String>,
    helper_set: Option<Instant>,
    helper_duration: Option<Duration>,
    escape_press_time: Option<Instant>,

    // method info bottom right
    method: ToolCallMethod,

    // bottom helper
    help: Option<HelpArea>,
    cmdnav: CommandNav,

    history: Vec<String>,
    history_index: usize,
}

impl Default for InputArea<'_> {
    fn default() -> Self {
        Self {
            agent_running: false,
            input: TextArea::default(),
            placeholder: "? for shortcuts".to_string(),
            current_draft: None,
            animation_start: None,
            status_message: None,
            last_keystroke_time: None,
            pending_enter: None,
            helper_msg: None,
            helper_set: None,
            helper_duration: None,
            escape_press_time: None,
            method: ToolCallMethod::FunctionCall,
            help: None,
            cmdnav: CommandNav {},
            history: Vec::new(),
            history_index: 0,
        }
    }
}

impl InputArea<'_> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_history(&mut self, history: Vec<String>) {
        self.history = history;
        self.history_index = self.history.len();
    }
}

/// method info bottom right
impl InputArea<'_> {
    pub fn set_tool_call_method(&mut self, method: ToolCallMethod) {
        self.method = method;
    }

    pub fn method_str(&self) -> &str {
        match self.method {
            ToolCallMethod::Auto => "🛠️ tool call try all methods",
            ToolCallMethod::FunctionCall => "🛠️ function call (auto)",
            ToolCallMethod::FunctionCallRequired => "🛠️ function call (required)",
            ToolCallMethod::StructuredOutput => "🛠️ structured output",
            ToolCallMethod::Parsing => "🛠️ parsing",
        }
    }
}

/// alert message in yellow, top left
impl InputArea<'_> {
    pub fn set_agent_running(&mut self, running: bool) {
        self.agent_running = running;
        if running {
            self.animation_start = Some(Instant::now());
        } else {
            self.status_message = None;
            self.animation_start = None;
        }
    }

    pub fn with_placeholder(mut self, placeholder: &str) -> Self {
        self.placeholder = placeholder.to_string();
        self
    }

    pub fn set_status(&mut self, text: &str) {
        self.status_message = Some(text.to_string());
    }

    pub fn is_animating(&self) -> bool {
        self.animation_start.is_some()
    }

    fn get_status_text(&self) -> String {
        if let Some(ref msg) = self.status_message {
            // Show status message if we have one (like "Task cancelled")
            format!(" {}", msg)
        } else if let Some(animation_start) = self.animation_start {
            // Show spinner when agent is working
            let spinner_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
            let elapsed = animation_start.elapsed().as_millis();
            let index = (elapsed / 100) % spinner_chars.len() as u128;
            format!(
                " {} Agent is working... (press esc to cancel)",
                spinner_chars[index as usize]
            )
        } else {
            // Agent is waiting for input, no status to show
            String::new()
        }
    }
}

/// status message bottom left
impl InputArea<'_> {
    pub fn alert_msg(&mut self, text: &str, duration: Duration) {
        self.helper_msg = Some(text.to_string());
        self.helper_set = Some(Instant::now());
        self.helper_duration = Some(duration);
    }

    pub fn check_pending_enter(&mut self) -> Option<UserAction> {
        if let Some(enter_time) = self.pending_enter {
            if enter_time.elapsed() >= Duration::from_millis(100) {
                self.pending_enter = None;

                if self.agent_running {
                    return Some(UserAction::Nope);
                }

                let lines = self.input.lines();
                if !lines[0].is_empty() {
                    let input = lines.join("\n");
                    self.history.push(input.clone());
                    self.history_index = self.history.len();

                    // Handle app commands vs agent input
                    self.input = TextArea::default();
                    if input.starts_with('/') {
                        return Some(UserAction::UserAppCommand { command: input });
                    } else {
                        return Some(UserAction::UserInput { input });
                    }
                }
            }
        }
        None
    }

    fn check_helper_msg(&mut self) -> String {
        // Check if escape message should be cleared after 1 second
        if let Some(helper_time) = self.helper_set {
            if helper_time.elapsed() >= self.helper_duration.unwrap() {
                self.helper_msg = None;
                self.helper_set = None;
                self.helper_duration = None;
                return String::new();
            }
        }

        // Return current helper message or empty string
        self.helper_msg.as_deref().unwrap_or("").to_string()
    }
}

/// event related
impl InputArea<'_> {
    fn move_cursor_to_end_of_text(&mut self) {
        for _ in 0..self.input.lines().len().saturating_sub(1) {
            self.input.move_cursor(tui_textarea::CursorMove::Down);
        }
        if let Some(last_line) = self.input.lines().last() {
            for _ in 0..last_line.len() {
                self.input.move_cursor(tui_textarea::CursorMove::Forward);
            }
        }
    }

    fn load_historic_prompt(&mut self, index: usize) {
        if let Some(entry) = self.history.get(index) {
            self.input = TextArea::new(entry.lines().map(|s| s.to_string()).collect());
            self.move_cursor_to_end_of_text();
        }
    }

    pub async fn handle_event(&mut self, key_event: KeyEvent) -> UserAction {
        let now = Instant::now();
        self.last_keystroke_time = Some(now);

        // Convert any pending Enter to newline
        if self.pending_enter.is_some() {
            self.pending_enter = None;
            let fake_event = KeyEvent {
                code: KeyCode::Enter,
                modifiers: KeyModifiers::empty(),
                kind: key_event.kind,
                state: key_event.state,
            };
            let event: Input = Event::Key(fake_event).into();
            self.input.input(event);
        }

        match key_event.code {
            KeyCode::Char('?') if self.input.lines()[0].is_empty() && self.help.is_none() => {
                self.help = Some(HelpArea);
            }
            KeyCode::Esc => {
                if self.agent_running {
                    return UserAction::CancelTask;
                }

                // Handle escape key for input clearing
                if let Some(escape_time) = self.escape_press_time {
                    // Second escape within 1 second - clear input
                    if escape_time.elapsed() < Duration::from_secs(1) {
                        self.input = TextArea::default();
                        self.escape_press_time = None;
                        self.helper_msg = None;
                        return UserAction::Nope;
                    }
                }

                // First escape or escape after timeout - show message
                if !self.input.lines()[0].is_empty() {
                    self.escape_press_time = Some(now);
                    self.helper_set = Some(now);
                    self.helper_duration = Some(Duration::from_secs(1));
                    self.helper_msg = Some(" press esc again to clear".to_string());
                }
            }
            KeyCode::Char('v')
                if key_event.modifiers.contains(KeyModifiers::CONTROL)
                    || key_event.modifiers.contains(KeyModifiers::SUPER) =>
            {
                // Handle Ctrl+V or Cmd+V paste directly from clipboard
                if let Ok(mut ctx) = ClipboardContext::new() {
                    if let Ok(text) = ctx.get_contents() {
                        self.input.insert_str(text);
                        return UserAction::Nope;
                    }
                }
                // Fallback: let TextArea handle it normally
                let event: Input = Event::Key(key_event).into();
                self.input.input(event);
                return UserAction::Nope;
            }
            KeyCode::Enter => {
                // Alt+Enter creates a new line immediately
                if key_event.modifiers.contains(KeyModifiers::ALT) {
                    self.last_keystroke_time = Some(now);

                    // Create fake Enter event without Alt modifier for TextArea
                    let fake_event = KeyEvent {
                        code: KeyCode::Enter,
                        modifiers: KeyModifiers::empty(),
                        kind: key_event.kind,
                        state: key_event.state,
                    };
                    let event: Input = Event::Key(fake_event).into();
                    self.input.input(event);
                    return UserAction::Nope;
                }

                // Regular Enter - set pending and wait
                self.pending_enter = Some(now);
                return UserAction::Nope;
            }
            KeyCode::Up => {
                // Get current cursor position
                let (cursor_row, _) = self.input.cursor();
                let is_empty = self.input.lines().iter().all(|line| line.is_empty());

                // Navigate history only if:
                // 1. Input is empty, OR
                // 2. Cursor is at the first line
                if !self.history.is_empty()
                    && self.history_index > 0
                    && (is_empty || cursor_row == 0)
                {
                    if self.history_index == self.history.len() && !is_empty {
                        let current_text = self.input.lines().join("\n");
                        self.current_draft = Some(current_text);
                    }

                    self.history_index -= 1;
                    self.load_historic_prompt(self.history_index);
                } else if !is_empty && cursor_row > 0 {
                    self.input.move_cursor(tui_textarea::CursorMove::Up);
                }
            }
            KeyCode::Down => {
                // Get current cursor position
                let (cursor_row, _) = self.input.cursor();
                let is_empty = self.input.lines().iter().all(|line| line.is_empty());
                let line_count = self.input.lines().len();

                // Navigate history only if:
                // 1. Cursor is at the last line
                if !self.history.is_empty() && (is_empty || cursor_row == line_count - 1) {
                    if self.history_index < self.history.len() {
                        self.history_index += 1;
                        if self.history_index < self.history.len() {
                            self.load_historic_prompt(self.history_index);
                        } else {
                            // Restore draft or create empty input
                            if let Some(draft) = self.current_draft.take() {
                                self.input =
                                    TextArea::new(draft.lines().map(|s| s.to_string()).collect());
                                self.move_cursor_to_end_of_text();
                            } else {
                                self.input = TextArea::default();
                            }
                        }
                    }
                } else if !is_empty && cursor_row < line_count - 1 {
                    self.input.move_cursor(tui_textarea::CursorMove::Down);
                }
            }
            _ => {
                // Convert to ratatui event format for tui-textarea
                self.help = None;
                let event: Event = Event::Key(KeyEvent::from(key_event));
                let input: Input = event.into();
                self.input.input(input);
            }
        }
        UserAction::Nope
    }
}

/// drawing logic
impl InputArea<'_> {
    pub fn height(&self) -> u16 {
        // +2 for top/bottom borders
        // +N for lines inside input
        // +1 for helper text below input
        self.input.lines().len().max(1) as u16 + 4 + self.help.as_ref().map_or(0, |h| h.height())
    }

    pub fn draw(&mut self, f: &mut Frame, area: Rect) {
        let [status, input_area, helper, help_area] = Layout::vertical([
            Constraint::Length(1),
            Constraint::Length(self.height() - 2),
            Constraint::Length(1),
            Constraint::Length(self.help.as_ref().map_or(0, |h| h.height())),
        ])
        .areas(area);

        // status
        f.render_widget(
            Span::styled(self.get_status_text(), Style::default().fg(Color::Yellow)),
            status,
        );

        // Input - clone and apply block styling
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding {
                left: 1,
                right: 1,
                top: 0,
                bottom: 0,
            })
            .border_style(Style::default().fg(Color::DarkGray));
        //.border_style(Style::default().bold().fg(Color::Rgb(WAKE_YELLOW.0, WAKE_YELLOW.1, WAKE_YELLOW.2)));
        let inner = block.inner(input_area);
        f.render_widget(block, input_area);

        let [pad, prompt] =
            Layout::horizontal([Constraint::Length(2), Constraint::Fill(1)]).areas(inner);
        f.render_widget(format!(">"), pad);

        // Set placeholder and block
        self.input.set_placeholder_text("? for help");
        self.input
            .set_placeholder_style(Style::default().fg(Color::DarkGray));
        self.input.set_style(Style::default().fg(Color::White));
        self.input
            .set_cursor_style(Style::default().fg(Color::White).bg(
                if !self.input.lines()[0].is_empty() {
                    Color::White
                } else {
                    Color::Reset
                },
            ));
        self.input.set_cursor_line_style(Style::default());
        f.render_widget(&self.input, prompt);

        // Helper text area below input
        let [helper_left, _, helper_right] = Layout::horizontal([
            Constraint::Fill(1),
            Constraint::Fill(1),
            Constraint::Length(self.method_str().len() as u16),
        ])
        .areas(helper);

        let helper_text = self.check_helper_msg();
        f.render_widget(
            Span::styled(helper_text, Style::default().fg(Color::DarkGray).dim()),
            helper_left,
        );

        // Status
        f.render_widget(
            Span::styled(self.method_str(), Style::default().fg(Color::DarkGray)),
            helper_right,
        );

        // help
        if let Some(help) = &self.help {
            help.draw(f, help_area);
        }
    }
}
