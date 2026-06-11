use crate::grid::Grid;
use crate::input;
use crate::patterns;
use crate::render::Renderer;
use crossterm::event;
use rand::rngs::ThreadRng;
use std::io::Stdout;
use std::time::Duration;

#[derive(Clone, Copy, Debug)]
pub enum Mode {
    Running,
    Paused,
    Editing { cx: u16, cy: u16 },
}

pub struct App {
    pub grid: Grid,
    pub renderer: Renderer,
    pub mode: Mode,
    pub tick_count: u64,
    pub fps: u32,
    pub quit: bool,
    pub rng: ThreadRng,
}

impl App {
    pub fn new(cols: u16, rows: u16, fps: u32, edit_mode: bool) -> Self {
        let char_rows = rows.saturating_sub(1);
        let mut grid = Grid::new(cols as usize, (char_rows as usize) * 2);
        let renderer = Renderer::new(cols, char_rows);
        let mut rng = rand::thread_rng();
        let mode = if edit_mode {
            let cx = grid.w as u16 / 2;
            let cy = grid.h as u16 / 2;
            Mode::Editing { cx, cy }
        } else {
            patterns::scatter(&mut grid, &mut rng);
            Mode::Running
        };
        Self {
            grid,
            renderer,
            mode,
            tick_count: 0,
            fps,
            quit: false,
            rng,
        }
    }

    pub fn run(&mut self, out: &mut Stdout) -> std::io::Result<()> {
        self.renderer
            .render(out, &self.grid, &self.mode, self.tick_count, self.fps)?;
        while !self.quit {
            let timeout = Duration::from_millis(1000 / self.fps.max(1) as u64);
            if event::poll(timeout)? {
                let ev = event::read()?;
                input::handle(self, ev);
            } else if matches!(self.mode, Mode::Running) {
                self.tick();
            }
            self.renderer
                .render(out, &self.grid, &self.mode, self.tick_count, self.fps)?;
        }
        Ok(())
    }

    pub fn tick(&mut self) {
        self.grid.tick();
        self.tick_count += 1;
    }

    pub fn reset_scatter(&mut self) {
        self.grid.clear();
        patterns::scatter(&mut self.grid, &mut self.rng);
        self.tick_count = 0;
        self.mode = Mode::Running;
        self.renderer.invalidate();
    }

    pub fn enter_edit(&mut self, clear_first: bool) {
        if clear_first {
            self.grid.clear();
        }
        let cx = self.grid.w as u16 / 2;
        let cy = self.grid.h as u16 / 2;
        self.mode = Mode::Editing { cx, cy };
        self.renderer.invalidate();
    }

    pub fn handle_resize(&mut self, cols: u16, rows: u16) {
        let char_rows = rows.saturating_sub(1);
        let new_w = cols as usize;
        let new_h = (char_rows as usize) * 2;
        let was_editing = matches!(self.mode, Mode::Editing { .. });
        self.grid.resize(new_w, new_h);
        self.renderer.resize(cols, char_rows);
        if was_editing {
            self.mode = Mode::Editing {
                cx: self.grid.w as u16 / 2,
                cy: self.grid.h as u16 / 2,
            };
        } else {
            patterns::scatter(&mut self.grid, &mut self.rng);
            self.tick_count = 0;
            self.mode = Mode::Running;
        }
    }
}
