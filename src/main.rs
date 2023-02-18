use crossterm::event::{DisableMouseCapture, EnableMouseCapture};
use crossterm::terminal::{
    disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
};
use std::fs::{File};
use std::env;
use std::io;
use std::io::Write;
use tui::backend::CrosstermBackend;
use tui::widgets::{Block, Borders};
use tui::Terminal;
use tui_textarea::{Input, Key, TextArea};

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();
    let mut file = File::create(&args[1])?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    enable_raw_mode()?;
    crossterm::execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut term = Terminal::new(backend)?;

    let mut textarea = TextArea::default();
    textarea.set_block(
        Block::default()
            .borders(Borders::ALL)
            .title("Rust Editor"),
    );

    loop {
        term.draw(|f| {
            f.render_widget(textarea.widget(), f.size());
        })?;
        match crossterm::event::read()?.into() {
            Input {
                key: Key::Char('q'),
                ctrl: true,
                alt: false
            } => break,
            input => {
                textarea.input(input);
            }
        }
    }

    disable_raw_mode()?;
    crossterm::execute!(
        term.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    term.show_cursor()?;
    
    for line in textarea.lines() {
        file.write_all(line.as_bytes())?;
        file.write_all(b"\n")?;
    }
    file.sync_all()?;
    Ok(())
}
