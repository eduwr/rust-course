pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub fn subtract(left: usize, right: usize) -> usize {
    left - right
}
pub fn divide(left: usize, right: usize) -> usize {
    left / right
}
pub fn multiply(left: usize, right: usize) -> usize {
    left * right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum_values() {
        let result = add(1, 2);
        assert_eq!(result, 3);
    }
    #[test]
    fn it_should_subtract_values() {
        let result = subtract(3, 2);
        assert_eq!(result, 1);
    }
    #[test]
    fn it_should_divide_values() {
        let result = divide(4, 2);
        assert_eq!(result, 2);
    }
    #[test]
    fn it_should_multiply_values() {
        let result = multiply(3, 2);
        assert_eq!(result, 6);
    }
}
