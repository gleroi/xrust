pub fn square_of_sum(n : u32) -> u32 {
    let sum = n * (n + 1) / 2;
    return sum * sum;
}

pub fn sum_of_squares(n : u32) -> u32 {
    let mut sum = 0;
    for x in 0..n + 1 {
        sum += x * x;
    }
    return sum;
}

pub fn difference(n : u32) -> u32 {
    return square_of_sum(n) - sum_of_squares(n);
}
