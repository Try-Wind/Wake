use std::time::Duration;

use ansi_to_tui::IntoText;
use crossterm::event::{Event, KeyCode, KeyEvent, MouseEvent};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Color, Style, Stylize}, 
    symbols::border,
    text::{Line, Span, Text}, 
    widgets::{Block, Borders, List, ListDirection, ListItem, Padding, Paragraph, Widget}, 
    Frame
};
use wake_core::{agent::{events::PermissionRequest, output::PrettyFormatter, PermissionResponse}, tools::{ToolCall, ToolResult}};
use tui_textarea::{Input, TextArea};

use super::theme::WAKE_YELLOW;

pub enum PermissionModalAction {
    Nope,
    Response {
        request_id: String,
        choice: PermissionResponse
    }
}

#[derive(Clone)]
pub struct PermissionWidget<'a> {
    pub request_id: String,
    pub request: PermissionRequest,
    pub remaining_perms: usize,

    selected_index: usize,
    formatted_request: String,
    preview: TextArea<'a>
}

impl PermissionWidget<'_> {
    pub fn new(request_id: String, request: PermissionRequest, total: usize) -> Self {
        let formatter = PrettyFormatter::new();
        let formatted_request = formatter.format_toolcall(&request.call, request.preview.as_ref());
        let mut preview = TextArea::from(formatted_request.into_text().unwrap());
        preview.set_cursor_line_style(Style::reset());
        preview.set_cursor_style(Style::reset());
        
        Self {
            request_id,
            request,
            selected_index: 0,
            remaining_perms: total,

            formatted_request,
            preview
        }
    }


    pub fn move_up(&mut self) {
        self.selected_index = if self.selected_index == 0 { 2 } else { self.selected_index - 1 };
    }

    pub fn move_down(&mut self) {
        self.selected_index = (self.selected_index + 1) % 3;
    }

    pub fn get_selected(&self) -> PermissionResponse {
        match self.selected_index {
            0 => PermissionResponse::Allow,
            1 => PermissionResponse::AllowAlways,
            2 => PermissionResponse::Deny,
            _ => PermissionResponse::Deny,
        }
    }

    pub async fn handle_mouse_event(&mut self, mouse_event: MouseEvent) ->  PermissionModalAction {
        let event: Input = Event::Mouse(mouse_event).into();
        self.preview.input(event);
        PermissionModalAction::Nope   
    }

    pub async fn handle_key_event(&mut self, key_event: KeyEvent) ->  PermissionModalAction {
        match key_event.code {
            KeyCode::Up => {
                self.move_up();
                PermissionModalAction::Nope
            }
            KeyCode::Down => {
                self.move_down();
                PermissionModalAction::Nope
            }
            KeyCode::Enter => {
                let request_id = self.request_id.clone();
                let choice = self.get_selected();
                PermissionModalAction::Response { request_id, choice }
            }
            KeyCode::Esc => {
                let request_id = self.request_id.clone();
                let choice = PermissionResponse::Deny;
                PermissionModalAction::Response { request_id, choice }
            }
            _ => PermissionModalAction::Nope
        }
    }

    pub fn height(&self) -> u16 {
       4 // outer permission block 2 + 1 top padding
       + 2 // inner tool preview block 2 (0 padding)
       + self.formatted_request.lines().count() as u16  // preview content
       + 4 // allow, yolo, deny + 1 top space
    }

    pub fn draw(&self, f: &mut Frame, area: Rect) {
        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding{left: 1, right: 1, top: 1, bottom: 1})
            .border_style(Style::default().fg(Color::Cyan))
            .title(if self.remaining_perms > 1 {
                format!(" 🔐 Permission Required ({}/{}) ", 1, self.remaining_perms)
            } else {
                format!(" 🔐 Permission Required ")
            });    

        let inner = block.inner(area);
        f.render_widget(block, area);

        let [tool, modal] = Layout::vertical([Constraint::Length(self.formatted_request.lines().count() as u16 + 2), Constraint::Length(4)]).areas(inner);

        let call = self.request.call.clone();
        let tool_name = PrettyFormatter::capitalize_first(&call.tool_name);
        let context = PrettyFormatter::extract_primary_param(&call.parameters, &call.tool_name);
        let mut title = Line::from(vec![
            Span::styled("🔧 ", Color::White),
            Span::styled(tool_name,    Style::new().white().bold())
        ]);
        if let Some((_,ctx)) = context {
            title.push_span(Span::styled(format!("({})", ctx), Style::new().white()));
        };

        let block = Block::default()
            .borders(Borders::ALL)
            .border_set(border::ROUNDED)
            .padding(Padding{left: 1, right: 1, top: 0, bottom: 0})
            .title(title)
            .title_style(Style::default().fg(Color::White))
            .border_style(Style::default().fg(Color::DarkGray));        
    
        let inner = block.inner(tool);
        f.render_widget(block, tool);
        f.render_widget(&self.preview, inner);

        let items = ["Allow", "Allow all tools and don't ask again for this session", "Deny"];
        let mut lines = vec![Line::from("Do you want to run this tool?")];
        for (i,s) in items.into_iter().enumerate() {
            if i == self.selected_index {
                lines.push(Line::from(vec![
                    Span::styled("❯ ", Color::White),
                    Span::styled(s,    Color::White)
                ]));
            } else {
                lines.push(Line::from(vec![
                    Span::styled("  ", Color::DarkGray),
                    Span::styled(s,    Color::DarkGray)
                ]));
            };
        }
        let text = Text::from(lines);
        let p = Paragraph::new(text);
        f.render_widget(p, modal);
    }
}
