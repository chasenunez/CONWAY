use clap::Parser;
use crossterm::{
    cursor, execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::io::{self};

mod app;
mod grid;
mod input;
mod patterns;
mod render;

#[derive(Parser, Debug)]
#[command(name = "conway", about = "Conway's Game of Life in the terminal")]
struct Args {
    /// Start on a blank canvas in edit mode (no auto-scatter).
    #[arg(short, long)]
    edit: bool,
    /// Target ticks per second (1..=60).
    #[arg(long, default_value_t = 12)]
    fps: u32,
}

struct TerminalGuard;

impl TerminalGuard {
    fn enter() -> io::Result<Self> {
        terminal::enable_raw_mode()?;
        let mut out = io::stdout();
        execute!(out, EnterAlternateScreen, cursor::Hide)?;
        Ok(Self)
    }
}

impl Drop for TerminalGuard {
    fn drop(&mut self) {
        let mut out = io::stdout();
        let _ = execute!(out, cursor::Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
    }
}

fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let mut out = io::stdout();
        let _ = execute!(out, cursor::Show, LeaveAlternateScreen);
        let _ = terminal::disable_raw_mode();
        default_hook(info);
    }));
}

fn main() -> io::Result<()> {
    let args = Args::parse();
    let fps = args.fps.clamp(1, 60);

    install_panic_hook();
    let _guard = TerminalGuard::enter()?;

    let (cols, rows) = terminal::size()?;
    let mut out = io::stdout();
    let mut app = app::App::new(cols, rows, fps, args.edit);
    app.run(&mut out)?;
    Ok(())
}
