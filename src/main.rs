use std::thread::sleep;
use std::time::Duration;

use rand::prelude::*;

use tetromino_console::{
    move_shape,
    point,
    BoxGrid,
    Shape,
    TETROMINOES,
    shape_from_vec,
    rotations,
};

const X_LO: i32 = 0;
const X_HI: i32 = 10 - 1;
const Y_LO: i32 = 0;
const Y_HI: i32 = 20 - 1;

fn in_bounds(shape: &Shape) -> bool {
    shape.iter().all(|p|
        p.x >= X_LO &&
        p.x <= X_HI &&
        p.y >= Y_LO &&
        p.y <= Y_HI
    )
}

fn main() {
    let mut rng = thread_rng();

    let baseline = shape_from_vec(
        &(X_LO..=X_HI).flat_map(|x| (-3..=-1).map(|y| (x, y)).collect::<Vec<_>>()).collect::<Vec<_>>()
    );
    let top_bar = shape_from_vec(
        &(X_LO..=X_HI).flat_map(|x| (20..=21).map(|y| (x, y)).collect::<Vec<_>>()).collect::<Vec<_>>()
    );


    loop {
        let mut grid = BoxGrid::new();
        grid.set_shape(&baseline, 255);
        grid.set_shape(&top_bar, 255);
        for i in 0.. {
            let t = TETROMINOES.choose(&mut rng).unwrap();

            match find_fit_random(&t, &grid, &mut rng) {
                Some(fit) => {
                    let i = i as i32;
                    let color = (i + 1) as u8;
                    grid.set_shape(&fit, color);
                    print!("\x1B[2J\x1B[H");
                    print!("{}", grid.render());
                    sleep(Duration::from_millis(250));
                },
                None => break
            }
        }
    }


    //for (i, t) in TETROMINOES.iter().enumerate() {
    //}
}

fn all_positions(t: &Shape) -> Vec<Shape> {
    let mut result: Vec<Shape> = Vec::new();
    for t2 in rotations(&t) {
        for y in Y_LO..=Y_HI { 
            for x in X_LO..=X_HI {
                let t3 = move_shape(&t2, &point(x, y));
                if in_bounds(&t3) {
                    result.push(t3);
                }
            }
        }
    }
    result
}

fn all_positions_random<R: Rng>(t: &Shape, rng: &mut R) -> Vec<Shape> {

    let mut result: Vec<Shape> = Vec::new();

    let mut rots = rotations(&t);
    rots.shuffle(rng);

    let mut x_positions: Vec<_> = (X_LO..=X_HI).collect();
    x_positions.shuffle(rng);

    for y in Y_LO..=Y_HI {
        for x in x_positions.iter() {
            for t2 in rots.iter() {
                let t3 = move_shape(t2, &point(*x, y));
                if in_bounds(&t3) {
                    result.push(t3);
                }
            }
        }
    }
    result
}


fn find_fit_random<R: Rng>(t: &Shape, grid: &BoxGrid, rng: &mut R) -> Option<Shape> {
    for t2 in all_positions_random(&t, rng) {
        if grid.is_clear(&t2) {
            return Some(t2)
        }
    }
    None
}
