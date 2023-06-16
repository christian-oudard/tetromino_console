use tetromino_console::{point, Grid};

fn main() {
    let mut grid = Grid::new();

    grid.line(&point(0, 0), &point(1, 0), 1);
    grid.line(&point(1, 0), &point(1, 1), 1);
    grid.line(&point(1, 1), &point(0, 1), 1);
    grid.line(&point(0, 1), &point(0, 0), 1);

    println!("{}", grid.render());

}
