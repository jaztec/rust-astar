pub enum CellKind {
    Easy,
    Normal,
    Hard,
    Blocked,
    Start,
    Finish
}

pub struct Cell {
    y: u32,
    x: u32,
    passable: CellKind
}

fn distance(a: &Cell, b: &Cell) -> f64 {
    let x1 = a.x;
    let x2 = b.x;
    let y1 = a.y;
    let y2 = b.y;

    let mut dx = (x2 as f64 - x1 as f64).abs();
    let mut dy = (y2 as f64 - y1 as f64).abs();

    let min = std::cmp::min(dx as u64, dy as u64) as f64;
    let max = std::cmp::max(dx as u64, dy as u64) as f64;

    let diagonal_steps = min;
    let straight_steps: f64 = max - min;

    let sq: f64 = 2.0;
    sq.sqrt() * diagonal_steps + straight_steps
}

impl Cell {
    fn get_penalty(&self) -> Option<u32> {
        match self.passable {
            CellKind::Easy => Some(1),
            CellKind::Normal => Some(3),
            CellKind::Hard => Some(6),
            CellKind::Finish => Some(3),
            _ => None,
        }
    }

    fn get_heuristic(&self, target: &Cell) -> Option<f64> {
        Some(distance(self, target))
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    row_count: u32,
    col_count: u32
}

impl Grid {
    pub fn new(r: u32, c: u32) -> Grid {
        let mut rows: Vec<Vec<Cell>> = Vec::with_capacity(r as usize);
        for i in 0..r {
            let mut cols: Vec<Cell> = Vec::with_capacity(c as usize);
            for j in 0..c {
                cols.push(Cell{y: i, x: j, passable: CellKind::Normal });
            }
            rows.push(cols);
        }
        Grid {
            cells: rows,
            row_count: r,
            col_count: c
        }
    }

    pub fn print(self) {
        print!("|");
        for _ in 0..self.col_count {
            print!("-");
        }
        print!("|\n");
        for ref rows in &self.cells {
            print!("|");
            for ref col in rows.into_iter() {
                match col.passable {
                    CellKind::Blocked => print!("X"),
                    CellKind::Start => print!("S"),
                    CellKind::Finish => print!("F"),
                    CellKind::Hard => print!("^"),
                    CellKind::Easy => print!(">"),
                    _ => print!(" ")
                }
            }
            print!("|\n")
        }
        print!("|");
        for _ in 0..self.col_count {
            print!("-");
        }
        print!("|\n");
    }

    pub fn block(&mut self, x : u32, y: u32) -> std::io::Result<()> {
        self.set_cell(y, x, CellKind::Blocked)
    }

    pub fn set_start(&mut self, x : u32, y : u32) -> std::io::Result<()> {
        self.set_cell(y, x, CellKind::Start)
    }

    pub fn set_finish(&mut self, x : u32, y : u32) -> std::io::Result<()> {
        self.set_cell(y, x, CellKind::Finish)
    }

    pub fn set_cell(&mut self, y: u32, x: u32, kind: CellKind) -> std::io::Result<()> {
        self.check_cell(x, y)?;
        self.cells[y as usize][x as usize].passable = kind;
        Ok(())
    }

    fn check_cell(&self, x: u32, y: u32) -> std::io::Result<()> {
        if y >= self.row_count {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "row out of range"));
        }
        if x >= self.col_count {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "col out of range"));
        }
        Ok(())
    }
}