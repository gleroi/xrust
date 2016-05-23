pub fn hamming_distance(left: &str, right: &str) -> Result<usize, &'static str> {
    if left.len() != right.len() {
        return Err("inputs of different length");
    }
    
    let mut left_char = left.chars();
    let mut right_char = right.chars();

    let mut distance: usize = 0;
    while let Some(left_letter) = left_char.next() {
        if let Some(right_letter) = right_char.next() {
            println!("left: {}, right: {}", left_letter, right_letter);
            if left_letter != right_letter {
                distance += 1;
            }
        }
    }
    
    return Ok(distance);
}
