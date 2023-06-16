use tetromino_console::{boxc, offset, point, Grid, DIRECTIONS};

fn main() {
    let mut grid = Grid::new();

    grid.line(&point(0, 0), &point(1, 0), 1);
    grid.line(&point(1, 0), &point(1, 1), 1);
    grid.line(&point(1, 1), &point(0, 1), 1);
    grid.line(&point(0, 1), &point(0, 0), 1);

    // Determine grid bounds.
    let (mut min_x, mut max_x, mut min_y, mut max_y) = (0, 0, 0, 0);
    for (p1, p2) in grid.keys() {
        min_x = min_x.min(p1.x).min(p2.x);
        max_x = max_x.max(p1.x).max(p2.x);
        min_y = min_y.min(p1.y).min(p2.y);
        max_y = max_y.max(p1.y).max(p2.y);
    }

    // Render grid.
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p0 = point(x, y);
            let mut lines = vec![];
            for dir in DIRECTIONS {
                let p1 = offset(&p0, &dir);
                lines.push(grid.get(&p0, &p1));
            }
            print!("{}", boxc(&lines).unwrap())
        }
        println!();
    }
}
