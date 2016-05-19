pub fn is_leap_year(year : u32) -> bool {
    /// returns true if num is divisible by factor
    fn is_divisible(num : u32, factor : u32) -> bool {
        return num % factor == 0;
    }
    return is_divisible(year, 4) && (!is_divisible(year, 100) || is_divisible(year, 400));
}
