use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{prelude::*, widgets::*, Terminal};

const OPENERR: &str = "Failed to open Viewer.";
const CLOSEERR: &str = "Failed to close Viewer.";

pub fn show(text: &str) -> bool {
    // setup terminal
    let mut stdout = std::io::stdout();
    enable_raw_mode().expect(OPENERR);
    execute!(stdout, EnterAlternateScreen).expect(OPENERR);
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend).expect(OPENERR);

    // Draw
    let mut scroll = 0;
    let mut result = false;

    loop {
        terminal
            .draw(|f| {
                let chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([Constraint::Max(99), Constraint::Min(1)])
                    .split(f.size());

                // Render License
                let paragraph = Paragraph::new(text)
                    .wrap(Wrap::default())
                    .scroll((scroll, 0));

                f.render_widget(paragraph, chunks[0]);
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
                    scroll = scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    scroll += 1;
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

    result
}
