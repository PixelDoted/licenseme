use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, Terminal};

use crate::spdx::SPDXId;

const OPENERR: &str = "Failed to open Selector.";
const CLOSEERR: &str = "Failed to close Selector.";

pub fn show(options: Vec<SPDXId>) -> Option<String> {
    // setup terminal
    let mut stdout = std::io::stdout();
    enable_raw_mode().expect(OPENERR);
    execute!(stdout, EnterAlternateScreen).expect(OPENERR);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect(OPENERR);

    // Draw
    let mut state = ListState::default();
    let mut result = false;

    state.select(Some(0));

    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(99), Constraint::Min(1)])
                    .split(f.size());

                // Render Selector
                let items: Vec<ListItem> =
                    options.iter().map(|id| ListItem::new(id.name)).collect();
                let list = List::new(items).highlight_symbol(">");

                f.render_stateful_widget(list, chunks[0], &mut state);
                f.render_widget(Paragraph::new("'y' to confirm, 'n' to cancel"), chunks[1]);
            })
            .unwrap();

        // Poll
        if !event::poll(std::time::Duration::from_millis(250)).unwrap() {
            continue;
        }

        // Event
        if let Event::Key(key) = event::read().unwrap() {
            match key.code {
                KeyCode::Up => {
                    let selected = state.selected().unwrap();
                    if selected == 0 {
                        state.select(Some(options.len() - 1));
                    } else {
                        state.select(Some(selected - 1));
                    }
                }
                KeyCode::Down => {
                    let selected = state.selected().unwrap() + 1;
                    if selected == options.len() {
                        state.select(Some(0));
                    } else {
                        state.select(Some(selected));
                    }
                }

                KeyCode::Char('y') => {
                    result = true;
                    break;
                }
                KeyCode::Char('n') => {
                    break;
                }
                _ => {}
            }
        }
    }

    // restore terminal
    disable_raw_mode().expect(CLOSEERR);
    execute!(terminal.backend_mut(), LeaveAlternateScreen).expect(CLOSEERR);
    terminal.show_cursor().expect(CLOSEERR);

    if result {
        let selected = state.selected().unwrap();
        Some(options[selected].id.to_string())
    } else {
        None
    }
}
