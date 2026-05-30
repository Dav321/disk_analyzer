use crate::filetree::{FileNode, FileTree, NodeId};
use crossterm::event;
use crossterm::event::KeyCode;
use ratatui::layout::Rect;
use ratatui::prelude::{Constraint, HorizontalAlignment, Layout, Line, Modifier, Style, Stylize};
use ratatui::widgets::{
    Block, Cell, Row, Scrollbar, ScrollbarOrientation, ScrollbarState, Table, TableState,
};
use ratatui::{DefaultTerminal, Frame};
use std::io;
use std::time::{Duration, Instant};

#[derive(Debug)]
pub struct App {
    tree: FileTree,
    scroll: usize,
    folder: NodeId,
}

impl App {
    pub fn new(tree: FileTree) -> Self {
        Self {
            tree,
            scroll: 0,
            folder: 0,
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
                    KeyCode::Up => self.scroll = self.scroll.saturating_sub(1),
                    KeyCode::Down => self.scroll = self.scroll.saturating_add(1),
                    KeyCode::Right | KeyCode::Enter => {
                        let i = self.tree.children(self.folder)[self.scroll];
                        if let FileNode::Dir { .. } = self.tree.nodes[i] {
                            self.folder = i;
                        }
                    }
                    KeyCode::Left | KeyCode::Esc => {
                        if let FileNode::Dir { parent, .. } = self.tree.nodes[self.folder] {
                            if let Some(parent) = parent {
                                self.folder = parent;
                            }
                        }
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
        let instructions = Line::from(vec![
            " Scroll ".into(),
            "<↑/↓/Wheel> ".blue().bold(),
            " Back ".into(),
            "<←/Esc> ".blue().bold(),
            " Enter ".into(),
            "<→/Enter> ".blue().bold(),
            " Quit ".into(),
            "<Q> ".blue().bold(),
        ]);
        let block = Block::bordered().title_bottom(instructions);

        let header = ["", "Name", "Size", "Target"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(Style::default().add_modifier(Modifier::BOLD))
            .height(1);

        let rows = self.rows();
        let rows_len = rows.len();
        let scroll = self.scroll.min(rows_len.saturating_sub(1));
        let table = Table::new(
            rows,
            [
                Constraint::Length(1),
                Constraint::Fill(2),
                Constraint::Fill(1),
                Constraint::Fill(2),
            ],
        )
        .block(block)
        .header(header)
        .style(Style::default())
        .row_highlight_style(Style::default().add_modifier(Modifier::REVERSED));
        frame.render_stateful_widget(
            table,
            area,
            &mut TableState::default().with_selected(self.scroll),
        );

        let mut scroll_state = ScrollbarState::default()
            .position(scroll)
            .content_length(rows_len)
            .viewport_content_length(area.height as usize);

        frame.render_stateful_widget(
            Scrollbar::new(ScrollbarOrientation::VerticalRight)
                .begin_symbol(Some("↑"))
                .end_symbol(Some("↓")),
            area,
            &mut scroll_state,
        );

        self.scroll = scroll;
    }

    fn rows(&'_ self) -> Vec<Row<'_>> {
        self.tree
            .children(self.folder)
            .iter()
            .map(|i| {
                let node = &self.tree.nodes[*i];
                let size = node.size_str();
                let name = node.name();
                match node {
                    FileNode::File { .. } => Row::new([" ".to_string(), name, size, "".to_string()]),
                    FileNode::Dir { .. } => Row::new(["/".to_string(), name, size, "".to_string()]),
                    FileNode::Symlink { target, .. } => Row::new(["-".to_string(), name, size, target.to_owned()]),
                }
            })
            .collect()
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
