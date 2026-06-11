use crate::grid::Grid;
use rand::Rng;

pub struct Pattern {
    #[allow(dead_code)]
    pub name: &'static str,
    pub cells: Vec<(i16, i16)>,
    pub w: i16,
    pub h: i16,
    pub weight: u32,
}

fn parse(name: &'static str, art: &str, weight: u32) -> Pattern {
    let mut cells = Vec::new();
    let mut max_x = 0i16;
    let mut max_y = 0i16;
    for (y, line) in art.lines().enumerate() {
        for (x, ch) in line.chars().enumerate() {
            if ch == 'O' || ch == '#' || ch == '*' {
                cells.push((x as i16, y as i16));
                if (x as i16) > max_x {
                    max_x = x as i16;
                }
                if (y as i16) > max_y {
                    max_y = y as i16;
                }
            }
        }
    }
    Pattern {
        name,
        cells,
        w: max_x + 1,
        h: max_y + 1,
        weight,
    }
}

pub fn catalog() -> Vec<Pattern> {
    vec![
        // still lifes — rare, mostly visual filler
        parse("block", concat!("OO\n", "OO"), 1),
        parse("beehive", concat!(".OO.\n", "O..O\n", ".OO."), 1),
        parse("loaf", concat!(".OO.\n", "O..O\n", ".O.O\n", "..O."), 1),
        parse("boat", concat!("OO.\n", "O.O\n", ".O."), 1),
        parse("tub", concat!(".O.\n", "O.O\n", ".O."), 1),
        // oscillators
        parse("blinker", "OOO", 2),
        parse("toad", concat!(".OOO\n", "OOO."), 2),
        parse("beacon", concat!("OO..\n", "OO..\n", "..OO\n", "..OO"), 2),
        parse(
            "pulsar",
            concat!(
                "..OOO...OOO..\n",
                ".............\n",
                "O....O.O....O\n",
                "O....O.O....O\n",
                "O....O.O....O\n",
                "..OOO...OOO..\n",
                ".............\n",
                "..OOO...OOO..\n",
                "O....O.O....O\n",
                "O....O.O....O\n",
                "O....O.O....O\n",
                ".............\n",
                "..OOO...OOO..",
            ),
            2,
        ),
        // spaceships — the lifeblood of an interesting canvas
        parse("glider", concat!(".O.\n", "..O\n", "OOO"), 5),
        parse(
            "lwss",
            concat!(".O..O\n", "O....\n", "O...O\n", "OOOO."),
            4,
        ),
        parse(
            "mwss",
            concat!("...O..\n", ".O...O\n", "O.....\n", "O....O\n", "OOOOO."),
            3,
        ),
        parse(
            "hwss",
            concat!(
                "...OO..\n",
                ".O....O\n",
                "O......\n",
                "O.....O\n",
                "OOOOOO.",
            ),
            3,
        ),
        // gun — rare but spectacular
        parse(
            "gosper_glider_gun",
            concat!(
                "........................O...........\n",
                "......................O.O...........\n",
                "............OO......OO............OO\n",
                "...........O...O....OO............OO\n",
                "OO........O.....O...OO..............\n",
                "OO........O...O.OO....O.O...........\n",
                "..........O.....O.......O...........\n",
                "...........O...O....................\n",
                "............OO......................",
            ),
            2,
        ),
    ]
}

pub fn scatter(grid: &mut Grid, rng: &mut impl Rng) {
    let cat = catalog();
    if cat.is_empty() || grid.w < 10 || grid.h < 10 {
        return;
    }
    let cells = grid.w * grid.h;
    let attempts = (cells / 600).clamp(5, 80);

    let total_weight: u32 = cat.iter().map(|p| p.weight).sum();

    for _ in 0..attempts {
        let mut roll = rng.gen_range(0..total_weight);
        let mut chosen = &cat[0];
        for p in &cat {
            if roll < p.weight {
                chosen = p;
                break;
            }
            roll -= p.weight;
        }

        let pad: i16 = 3;
        let pw = chosen.w + pad;
        let ph = chosen.h + pad;
        if (pw as usize) >= grid.w || (ph as usize) >= grid.h {
            continue;
        }
        let ox = rng.gen_range(0..(grid.w as i16 - pw));
        let oy = rng.gen_range(0..(grid.h as i16 - ph));

        // Reject placements that would heavily overlap existing live cells.
        let mut overlap = 0usize;
        for (dx, dy) in &chosen.cells {
            let x = (ox + dx) as usize;
            let y = (oy + dy) as usize;
            if grid.get(x, y) {
                overlap += 1;
            }
        }
        if overlap * 5 > chosen.cells.len() * 2 {
            continue;
        }

        for (dx, dy) in &chosen.cells {
            let x = (ox + dx) as usize;
            let y = (oy + dy) as usize;
            grid.set(x, y, true);
        }
    }
}
