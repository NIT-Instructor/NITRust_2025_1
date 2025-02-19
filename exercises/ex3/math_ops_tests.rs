#[cfg(test)]
mod tests {
    use crate::exercises::ex3::{advanced_ops::math_advanced_ops::*, basic_ops::math_basic_ops::*};

    #[test]
    fn basic_ops_test() {
        assert_eq!(add(2, 3), 5);
        assert_eq!(subtract(3, 2), 1);
        assert_eq!(multiply(2, 3), 6);
        assert_eq!(divide(6, 3).unwrap(), 2);
    }

    #[test]
    fn advanced_ops_test() {
        assert_eq!(square(3), 9);
        assert_eq!(root(9.0).unwrap(), 3.0);
        assert_eq!(cube(3), 27);
    }
}
