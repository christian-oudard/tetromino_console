use tetromino_console::{move_shape, normalize, point, rotate_shape, tetrominoes, BoxGrid, Point};

fn main() {
    let tets = tetrominoes();
    let mut grid = BoxGrid::new();
    for (i, t) in tets.iter().enumerate() {
        let i = i as i32;
        let t = move_shape(t, &point(i * 5, 0));
        grid.set_shape(&t, i as u8);
    }
}


