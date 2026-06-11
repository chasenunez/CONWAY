use crate::app::{App, Mode};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};

pub fn handle(app: &mut App, event: Event) {
    match event {
        Event::Key(key) => handle_key(app, key),
        Event::Resize(cols, rows) => app.handle_resize(cols, rows),
        _ => {}
    }
}

fn handle_key(app: &mut App, key: KeyEvent) {
    if key.kind != KeyEventKind::Press {
        return;
    }
    if key.modifiers.contains(KeyModifiers::CONTROL) && key.code == KeyCode::Char('c') {
        app.quit = true;
        return;
    }
    match app.mode {
        Mode::Running | Mode::Paused => handle_sim_key(app, key),
        Mode::Editing { .. } => handle_edit_key(app, key),
    }
}

fn handle_sim_key(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Char(' ') => {
            app.mode = match app.mode {
                Mode::Running => Mode::Paused,
                _ => Mode::Running,
            };
        }
        KeyCode::Char('n') => {
            if matches!(app.mode, Mode::Paused) {
                app.tick();
            }
        }
        KeyCode::Char('r') => app.reset_scatter(),
        KeyCode::Char('+') | KeyCode::Char('=') => {
            app.fps = (app.fps + 2).min(60);
        }
        KeyCode::Char('-') | KeyCode::Char('_') => {
            app.fps = app.fps.saturating_sub(2).max(1);
        }
        KeyCode::Char('e') => app.enter_edit(false),
        KeyCode::Char('E') => app.enter_edit(true),
        _ => {}
    }
}

fn handle_edit_key(app: &mut App, key: KeyEvent) {
    let (cx, cy) = if let Mode::Editing { cx, cy } = app.mode {
        (cx, cy)
    } else {
        return;
    };
    let w = app.grid.w as u16;
    let h = app.grid.h as u16;
    if w == 0 || h == 0 {
        return;
    }
    match key.code {
        KeyCode::Char('q') => app.quit = true,
        KeyCode::Esc => app.mode = Mode::Paused,
        KeyCode::Enter => app.mode = Mode::Running,
        KeyCode::Char('c') => {
            app.grid.clear();
            app.renderer.invalidate();
        }
        KeyCode::Char(' ') => app.grid.toggle(cx as usize, cy as usize),
        KeyCode::Char('h') | KeyCode::Left => {
            let new_cx = if cx == 0 { w - 1 } else { cx - 1 };
            app.mode = Mode::Editing { cx: new_cx, cy };
        }
        KeyCode::Char('l') | KeyCode::Right => {
            let new_cx = (cx + 1) % w;
            app.mode = Mode::Editing { cx: new_cx, cy };
        }
        KeyCode::Char('k') | KeyCode::Up => {
            let new_cy = if cy == 0 { h - 1 } else { cy - 1 };
            app.mode = Mode::Editing { cx, cy: new_cy };
        }
        KeyCode::Char('j') | KeyCode::Down => {
            let new_cy = (cy + 1) % h;
            app.mode = Mode::Editing { cx, cy: new_cy };
        }
        _ => {}
    }
}
