/* Die-rolling and card-drawing functions.

Most of rogue's randomness seems to be even flat probabilities, ie rnd(10) is 1d10 - 1.
*/
#![allow(dead_code)]

// I'd like to specify a range type here but finding out how wasn't worth it
pub fn roll_die(lowest: u8, highest: u8) -> u8 {
    rand::random_range(lowest..=highest)
}

#[cfg(test)]
mod tests {
    #![allow(non_snake_case)]
    use super::*;

    #[test]
    fn test_random() {
        for _ in 1..1000 {
            let x = roll_die(1, 10);
            assert!(1 <= x && x <= 10); // wow, can't chain these operators in rust
        }
    }
}
