// TODO: Check all dates! Function to convert to DB and back, that only hours are saved.
//       They also should always use NaiveDate

use ratatui::backend::CrosstermBackend;
use ratatui::Terminal;
use rusty_gains::app::{App, AppResult};
use rusty_gains::event::{Event, EventHandler};
use rusty_gains::handler::handle_key_events;
use rusty_gains::tui::Tui;
use std::io;

fn main() -> AppResult<()> {
    // Create an application.
    let mut app = App::new();

    // Initialize the terminal user interface.
    let backend = CrosstermBackend::new(io::stderr());
    let terminal = Terminal::new(backend)?;
    let events = EventHandler::new(1000);
    let mut tui = Tui::new(terminal, events);
    tui.init()?;

    // Start the main loop.
    while app.running {
        // Render the user interface.
        tui.draw(&mut app)?;
        // Handle events.
        match tui.events.next()? {
            Event::Tick => app.tick(),
            Event::Key(key_event) => handle_key_events(key_event, &mut app)?,
            Event::Mouse(_) => {}
            Event::Resize(_, _) => {
                println!("Resize event");
            }
        }
    }

    // Exit the user interface.
    tui.exit()?;
    Ok(())
}
