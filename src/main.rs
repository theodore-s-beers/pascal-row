#![warn(clippy::pedantic, clippy::nursery)]
#![allow(clippy::manual_is_multiple_of)]

fn main() {
    let row = nth_pascal_row(32);
    println!("{row:?}");
}

fn nth_pascal_row(n: usize) -> Vec<u64> {
    let mut row_a = vec![1; n + 1];
    let mut row_b = row_a.clone();

    let mut curr: usize;
    let mut frontier = 2;

    while frontier <= n {
        curr = 1;

        while curr < frontier {
            if frontier % 2 == 0 {
                row_a[curr] = row_b[curr - 1] + row_b[curr];
            } else {
                row_b[curr] = row_a[curr - 1] + row_a[curr];
            }

            curr += 1;
        }

        frontier += 1;
    }

    if n % 2 == 0 { row_a } else { row_b }
}
