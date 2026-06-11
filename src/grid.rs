pub struct Grid {
    pub w: usize,
    pub h: usize,
    front: Vec<u8>,
    back: Vec<u8>,
}

impl Grid {
    pub fn new(w: usize, h: usize) -> Self {
        Self {
            w,
            h,
            front: vec![0; w * h],
            back: vec![0; w * h],
        }
    }

    #[inline]
    fn idx(&self, x: usize, y: usize) -> usize {
        y * self.w + x
    }

    pub fn get(&self, x: usize, y: usize) -> bool {
        if x >= self.w || y >= self.h {
            return false;
        }
        self.front[self.idx(x, y)] != 0
    }

    pub fn set(&mut self, x: usize, y: usize, alive: bool) {
        if x >= self.w || y >= self.h {
            return;
        }
        let i = self.idx(x, y);
        self.front[i] = alive as u8;
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        if x >= self.w || y >= self.h {
            return;
        }
        let i = self.idx(x, y);
        self.front[i] ^= 1;
    }

    pub fn clear(&mut self) {
        self.front.fill(0);
        self.back.fill(0);
    }

    pub fn resize(&mut self, w: usize, h: usize) {
        self.w = w;
        self.h = h;
        self.front = vec![0; w * h];
        self.back = vec![0; w * h];
    }

    pub fn tick(&mut self) {
        if self.w == 0 || self.h == 0 {
            return;
        }
        let w = self.w;
        let h = self.h;
        for y in 0..h {
            let yn = if y == 0 { h - 1 } else { y - 1 };
            let ys = if y + 1 == h { 0 } else { y + 1 };
            for x in 0..w {
                let xw = if x == 0 { w - 1 } else { x - 1 };
                let xe = if x + 1 == w { 0 } else { x + 1 };
                let n = self.front[yn * w + xw]
                    + self.front[yn * w + x]
                    + self.front[yn * w + xe]
                    + self.front[y * w + xw]
                    + self.front[y * w + xe]
                    + self.front[ys * w + xw]
                    + self.front[ys * w + x]
                    + self.front[ys * w + xe];
                let alive = self.front[y * w + x] != 0;
                self.back[y * w + x] = match (alive, n) {
                    (true, 2) | (true, 3) | (false, 3) => 1,
                    _ => 0,
                };
            }
        }
        std::mem::swap(&mut self.front, &mut self.back);
    }
}
