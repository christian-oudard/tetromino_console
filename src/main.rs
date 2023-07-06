use tetromino_console::{point, BoxGrid};

fn main() {
    let mut grid = BoxGrid::new();

    grid.set(&point(0, 0), 1);
    grid.set(&point(1, 0), 1);
    grid.set(&point(2, 0), 1);
    grid.set(&point(2, 1), 1);

    grid.set(&point(0, 1), 2);
    grid.set(&point(1, 1), 2);
    grid.set(&point(0, 2), 2);
    grid.set(&point(1, 2), 2);

    grid.set(&point(1, 3), 3);
    grid.set(&point(2, 3), 3);
    grid.set(&point(2, 2), 3);
    grid.set(&point(3, 2), 3);

    println!("{}", grid.render());
}
