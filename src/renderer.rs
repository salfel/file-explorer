use std::io;

use ratatui::{
    crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Direction, Layout},
    widgets::ListState,
    Frame,
};

use crate::{navigator::Navigator, tui, widgets::file_tree::FileTree};

pub struct Renderer {
    navigator: Navigator,
    dir_state: ListState,
    exit: bool,
}

impl Renderer {
    pub fn new() -> Renderer {
        let navigator = Navigator::new();
        Renderer {
            navigator,
            dir_state: ListState::default(),
            exit: false,
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
        let i = match self.dir_state.selected() {
            Some(i) => {
                if i >= self.navigator.current.children.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.dir_state.select(Some(i));
    }

    fn previous(&mut self) {
        let i = match self.dir_state.selected() {
            Some(i) => {
                if i == 0 {
                    self.navigator.current.children.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.dir_state.select(Some(i));
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

    pub fn render_frame(&mut self, frame: &mut Frame) {
        let layout = Layout::default()
            .direction(Direction::Horizontal)
            .constraints(vec![
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
                Constraint::Ratio(1, 3),
            ])
            .split(frame.size());

        let file_tree = FileTree::new(&self.navigator.current.children);

        frame.render_stateful_widget(file_tree, layout[0], &mut self.dir_state);
    }
}
