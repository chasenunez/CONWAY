use crate::app::Mode;
use crate::grid::Grid;
use crossterm::{
    cursor::{self, MoveTo},
    queue,
    style::{Color, Print, ResetColor, SetBackgroundColor, SetForegroundColor},
};
use std::io::{Stdout, Write};

#[derive(Copy, Clone, PartialEq, Eq, Default)]
struct CharCell {
    top: bool,
    bot: bool,
}

pub struct Renderer {
    pub cols: u16,
    pub char_rows: u16,
    prev: Vec<CharCell>,
    next: Vec<CharCell>,
    force_full: bool,
}

impl Renderer {
    pub fn new(cols: u16, char_rows: u16) -> Self {
        let size = cols as usize * char_rows as usize;
        Self {
            cols,
            char_rows,
            prev: vec![CharCell::default(); size],
            next: vec![CharCell::default(); size],
            force_full: true,
        }
    }

    pub fn resize(&mut self, cols: u16, char_rows: u16) {
        self.cols = cols;
        self.char_rows = char_rows;
        let size = cols as usize * char_rows as usize;
        self.prev = vec![CharCell::default(); size];
        self.next = vec![CharCell::default(); size];
        self.force_full = true;
    }

    pub fn invalidate(&mut self) {
        self.force_full = true;
    }

    fn idx(&self, col: u16, row: u16) -> usize {
        row as usize * self.cols as usize + col as usize
    }

    pub fn render(
        &mut self,
        out: &mut Stdout,
        grid: &Grid,
        mode: &Mode,
        tick: u64,
        fps: u32,
    ) -> std::io::Result<()> {
        for row in 0..self.char_rows {
            for col in 0..self.cols {
                let top = grid.get(col as usize, (row as usize) * 2);
                let bot = grid.get(col as usize, (row as usize) * 2 + 1);
                let i = self.idx(col, row);
                self.next[i] = CharCell { top, bot };
            }
        }

        let mut fg_set = false;
        for row in 0..self.char_rows {
            for col in 0..self.cols {
                let i = self.idx(col, row);
                let cell = self.next[i];
                if !self.force_full && cell == self.prev[i] {
                    continue;
                }
                queue!(out, MoveTo(col, row))?;
                match (cell.top, cell.bot) {
                    (false, false) => {
                        if fg_set {
                            queue!(out, ResetColor)?;
                            fg_set = false;
                        }
                        queue!(out, Print(" "))?;
                    }
                    (true, true) => {
                        if !fg_set {
                            queue!(out, SetForegroundColor(Color::White))?;
                            fg_set = true;
                        }
                        queue!(out, Print("█"))?;
                    }
                    (true, false) => {
                        if !fg_set {
                            queue!(out, SetForegroundColor(Color::White))?;
                            fg_set = true;
                        }
                        queue!(out, Print("▀"))?;
                    }
                    (false, true) => {
                        if !fg_set {
                            queue!(out, SetForegroundColor(Color::White))?;
                            fg_set = true;
                        }
                        queue!(out, Print("▄"))?;
                    }
                }
            }
        }
        queue!(out, ResetColor)?;

        self.draw_status(out, mode, tick, fps)?;

        match mode {
            Mode::Editing { cx, cy } => {
                let row = (cy / 2).min(self.char_rows.saturating_sub(1));
                queue!(out, cursor::Show, MoveTo(*cx, row))?;
            }
            _ => {
                queue!(out, cursor::Hide)?;
            }
        }

        out.flush()?;
        std::mem::swap(&mut self.prev, &mut self.next);
        self.force_full = false;
        Ok(())
    }

    fn draw_status(
        &self,
        out: &mut Stdout,
        mode: &Mode,
        tick: u64,
        fps: u32,
    ) -> std::io::Result<()> {
        let row = self.char_rows;
        let text = match mode {
            Mode::Running => format!(
                " tick {} | running | {} fps | space=pause r=reset e=edit q=quit",
                tick, fps
            ),
            Mode::Paused => format!(
                " tick {} | PAUSED  | {} fps | space=resume n=step r=reset e=edit q=quit",
                tick, fps
            ),
            Mode::Editing { cx, cy } => format!(
                " EDIT | cursor ({},{}) | arrows=move space=toggle enter=run c=clear esc=back",
                cx, cy
            ),
        };
        let width = self.cols as usize;
        let mut s: String = text.chars().take(width).collect();
        while s.chars().count() < width {
            s.push(' ');
        }
        queue!(
            out,
            MoveTo(0, row),
            SetForegroundColor(Color::Black),
            SetBackgroundColor(Color::White),
            Print(s),
            ResetColor
        )?;
        Ok(())
    }
}
