mod structures;

fn main() -> std::io::Result<()> {
    let mut grid = structures::Grid::new(10, 20);
    grid.block(5, 5)?;
    grid.block(5, 6)?;
    grid.block(6, 6)?;

    grid.set_start(1, 1)?;
    grid.set_finish(9, 9)?;

    grid.set_cell(3, 4, structures::CellKind::Hard)?;
    grid.set_cell(4, 4, structures::CellKind::Hard)?;
    grid.set_cell(5, 4, structures::CellKind::Easy)?;
    grid.set_cell(6, 4, structures::CellKind::Easy)?;

    grid.set_cell(9, 4, structures::CellKind::Easy)?;
    grid.set_cell(9, 5, structures::CellKind::Hard)?;
    grid.set_cell(8, 4, structures::CellKind::Hard)?;
    grid.set_cell(8, 5, structures::CellKind::Hard)?;

    grid.print();

    Ok(())
}
