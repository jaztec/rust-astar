use std::collections::BTreeMap;

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

    fn get_heuristic(&self, target: &Cell) -> Option<u32> {
        Some(distance(self, target) as u32)
    }
}

pub struct Grid {
    cells: Vec<Vec<Cell>>,
    row_count: u32,
    col_count: u32,
    start_cell: (u32, u32),
    finish_cell: (u32, u32)
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
            col_count: c,
            start_cell: (0, 0),
            finish_cell: (0, 0),
        }
    }

    pub fn print(&self) {
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
        self.start_cell.0 = y;
        self.start_cell.1 = x;
        self.set_cell(y, x, CellKind::Start)
    }

    pub fn set_finish(&mut self, x : u32, y : u32) -> std::io::Result<()> {
        self.finish_cell.0 = y;
        self.finish_cell.1 = x;
        self.set_cell(y, x, CellKind::Finish)
    }

    pub fn set_cell(&mut self, y: u32, x: u32, kind: CellKind) -> std::io::Result<()> {
        self.check_cell(x, y)?;
        self.cells[y as usize][x as usize].passable = kind;
        Ok(())
    }

    pub fn get_cell(&self, x : u32, y : u32) -> Option<&Cell> {
        match self.check_cell(x, y) {
            Ok(()) => Some(&self.cells[y as usize][x as usize]),
            Err(err) => None
        }
    }

    pub fn get_adjacent(&self, cell: &Cell) -> Vec<&Cell> {
        let mut list: Vec<&Cell> = vec![];
        for y in cell.y-1..cell.y+1 {
            for x in cell.x-1..cell.x+1 {
                match self.get_cell(x, y) {
                    Some(cell) => list.push(cell),
                    _ => {}
                }
            }
        }
        list
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

pub fn calculate_route(grid: &Grid) -> std::io::Result<Vec<(u32, u32)>> {
    let start = grid.get_cell(grid.start_cell.1, grid.start_cell.0).unwrap();
    let finish = grid.get_cell(grid.finish_cell.1, grid.finish_cell.0).unwrap();
    let mut route: Vec<(u32, u32)> = vec![];
    let mut list: BTreeMap<u32, &Cell> = BTreeMap::new();

    list.insert(start.get_heuristic(finish).unwrap(), start);

    process(grid, &route, &list)?;

    Ok(route)
}

fn process(grid: &Grid, mut route: &Vec<(u32, u32)>, list: &BTreeMap<u32, &Cell>) -> std::io::Result<()> {

    Ok(())
}