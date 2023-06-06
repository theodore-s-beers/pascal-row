#![warn(clippy::pedantic, clippy::nursery)]

fn main() {
    let row = get_triangle_row(12);
    println!("{row:?}");
}

fn get_triangle_row(n: usize) -> Vec<u64> {
    let length = n + 1;

    let mut row_a = vec![1; length];
    let mut row_b = row_a.clone();

    let mut right = 2;

    while right < length {
        for i in 1..right {
            match right % 2 {
                0 => row_a[i] = row_b[i - 1] + row_b[i],
                _ => row_b[i] = row_a[i - 1] + row_a[i],
            }
        }

        right += 1;
    }

    match n % 2 {
        0 => row_a,
        _ => row_b,
    }
}
