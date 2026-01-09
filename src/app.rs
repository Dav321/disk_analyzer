use crate::filetree::FileTree;
use crossterm::event;
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, HorizontalAlignment, Layout, Line, Modifier, Stylize};
use ratatui::widgets::{Block, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct App {
    tree: FileTree,
    scroll_state: ScrollbarState,
    scroll: usize,
}

impl App {
    pub fn new(tree: FileTree) -> Self {
        Self {
            tree,
            scroll_state: ScrollbarState::default(),
            scroll: 0,
        }
    }

    pub fn run(&mut self, terminal: &mut DefaultTerminal) -> io::Result<()> {
        let tick_rate = Duration::from_millis(1000 / 60);
        let mut last_tick = Instant::now();

        loop {
            terminal.draw(|frame| self.render(frame))?;

            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if !event::poll(timeout)? {
                last_tick = Instant::now();
                continue;
            }
            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Up => {
                        self.scroll = self.scroll.saturating_sub(1);
                        self.scroll_state = self.scroll_state.position(self.scroll);
                    }
                    KeyCode::Down => {
                        self.scroll = self.scroll.saturating_add(1);
                        self.scroll_state = self.scroll_state.position(self.scroll);
                    }
                    _ => {}
                }
            }
        }
    }

    pub fn render(&mut self, frame: &mut Frame) {
        let area = frame.area();

        let chunks = Layout::vertical([Constraint::Length(1), Constraint::Fill(1)]).split(area);

        self.render_header(frame, chunks[0]);
        self.render_content(frame, chunks[1])
    }

    fn render_content(&mut self, frame: &mut Frame, area: Rect) {
        let text = format!("{}", self.tree);
        let line_count = text.lines().count();

        let instructions = Line::from(vec![" Quit ".into(), "<Q> ".blue().bold()]);
        let block = Block::bordered().title_bottom(instructions);
        let paragraph = Paragraph::new(text)
            .block(block)
            .scroll((self.scroll as u16, 0));
        frame.render_widget(paragraph, area);

        self.scroll_state = self
            .scroll_state
            .content_length(line_count)
            .viewport_content_length(area.height as usize);

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut self.scroll_state,
        );
    }

    fn render_header(&self, frame: &mut Frame, area: Rect) {
        let title = Block::new()
            .title_alignment(HorizontalAlignment::Center)
            .title(vec![
                "Disk Analyzer".add_modifier(Modifier::BOLD),
                env!("CARGO_PKG_VERSION").reset(),
            ]);
        frame.render_widget(title, area);
    }
}
