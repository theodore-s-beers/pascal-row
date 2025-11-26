#![warn(clippy::pedantic, clippy::nursery)]

use pascal_row::nth_pascal_row;

fn main() {
    let n: usize = 16;
    let mut row = vec![0u64; n + 1];

    nth_pascal_row(n, &mut row).unwrap();
    println!("{row:?}");
}
