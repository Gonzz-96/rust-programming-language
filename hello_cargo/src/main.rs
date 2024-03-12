fn main() {
    println!("Hello, world!");
}

pub fn adder(left: usize, right: usize) -> usize {
    return left + right;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn sum_is_ok() {
        let left: usize  = 34;
        let right: usize = 16;

        let result = adder(left, right);

        assert_eq!(result, 50);
    }
}
