use std::io;
use std::rc::Rc;

use crate::tui;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    prelude::StatefulWidget,
    style::{Modifier, Style, Stylize},
    symbols::border,
    text::ToLine,
    widgets::{block::Title, Block, List, ListDirection, ListState, Paragraph, Widget},
    Frame,
};

use crate::navigator::Navigator;

#[derive(Debug)]
pub struct FileTree {
    exit: bool,
    navigator: Rc<Navigator>,
    state: ListState,
}

impl FileTree {
    pub fn new(navigation: Rc<Navigator>) -> FileTree {
        FileTree {
            exit: false,
            navigator: Rc::clone(&navigation),
            state: ListState::default(),
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.navigator.current.children.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.navigator.current.children.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size());

        let list = List::new(
            self.navigator
                .current
                .children
                .iter()
                .map(|entity| entity.name.to_string()),
        )
        .block(Block::bordered().title("File Tree"))
        .highlight_style(Style::default().bold().on_dark_gray())
        .repeat_highlight_symbol(true)
        .direction(ListDirection::TopToBottom);

        frame.render_stateful_widget(list, layout[0], &mut self.state);
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event);
            }
            _ => {}
        }

        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('k') => self.previous(),
            KeyCode::Char('j') => self.next(),
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}
