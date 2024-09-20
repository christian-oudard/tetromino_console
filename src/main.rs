use tetromino_console::{move_shape, point, tetrominoes, BoxGrid};

fn main() {
    let tets = tetrominoes();
    let mut grid = BoxGrid::new();
    for (i, t) in tets.iter().enumerate() {
        let i = i as i32;
        let t = move_shape(t, &point(i * 4, 0));
        let color = (i + 1) as u8;
        grid.set_shape(&t, color);
    }
    print!("{}", grid.render());
}
