#![allow(unused_imports)]
use engine::life_board::{BoardPoint, LifeBoard};
use engine::pattern::Pattern;
use std::io;
use std::time::{Duration, Instant};

fn main() {
    // let mut life_board = engine::new_fixed_vector_board();
    // let mut life_board = engine::new_dynamic_vector_board();
    // let mut life_board = engine::new_dynamic_array2d_board();
    let mut life_board = engine::new_fixed_bitfield_board();
    life_board.draw_pattern(&Pattern::ACORN(), &BoardPoint::new(128, 128));

    println!("Beginning test...");
    let start = Instant::now();

    for _i in 0..5000 {
        life_board.step_one();
    }

    let end = Instant::now();
    let elapsed = end.duration_since(start);
    println!("Test done!");
    println!("Elapsed time={}s", elapsed.as_secs_f32());

    let stats_text = life_board
        .get_stats()
        .iter()
        .map(|(k, v)| format!("{}: {}", k, v))
        .collect::<Vec<String>>()
        .join("\n");
    println!("Board stats:\n{}", stats_text);
}
