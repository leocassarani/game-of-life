extern crate rand;
extern crate termsize;

use rand::Rng;
use std::fmt;
use std::fmt::Display;
use std::thread::sleep;
use std::time::Duration;
use termsize::Size;

fn main() {
    let size = termsize::get();

    let (width, height) = if let Some(Size{rows, cols}) = size {
        (cols as isize, rows as isize)
    } else {
        (20, 20)
    };

    let mut g = Grid::new(width, height);

    loop {
        clear();
        print!("\r{}", g);
        g = g.tick();
        sleep(Duration::from_millis(100));
    }
}

fn clear() {
    print!("{}[2J", 27 as char);
}

#[derive(Clone)]
struct Grid {
    width: isize,
    height: isize,
    cells: Vec<bool>
}

impl Grid {
    pub fn new(width: isize, height: isize) -> Grid {
        let mut rng = rand::thread_rng();
        let cells = (0..(width * height)).map(|_| rng.gen()).collect::<Vec<bool>>();

        Grid {
            cells: cells,
            width: width,
            height: height
        }
    }

    pub fn tick(&mut self) -> Grid {
        let mut cells = Vec::new();

        for y in 0..self.height {
            for x in 0..self.width {
                let cur = self.cell_at(x, y);
                let neighbours = self.neighbour_count(x, y);
                let alive = if cur {
                    neighbours == 2 || neighbours == 3
                } else {
                    neighbours == 3
                };
                cells.push(alive);
            }
        }

        Grid {
            cells: cells, ..*self
        }
    }

    fn cell_at(&self, x: isize, y: isize) -> bool {
        if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false
        }
        self.cells[(y * self.width + x) as usize]
    }

    fn neighbour_count(&self, x: isize, y: isize) -> usize {
        [
            self.cell_at(x-1, y-1),
            self.cell_at(x-1, y),
            self.cell_at(x-1, y+1),
            self.cell_at(x, y-1),
            self.cell_at(x, y+1),
            self.cell_at(x+1, y-1),
            self.cell_at(x+1, y),
            self.cell_at(x+1, y+1),
        ].into_iter().filter(|&&x| x).count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.cell_at(x, y) {
                    write!(f, "*").unwrap();
                } else {
                    write!(f, " ").unwrap();
                }
            }

            if y < self.height - 1 {
                write!(f, "\n").unwrap()
            }
        }
        Ok(())
    }
}
