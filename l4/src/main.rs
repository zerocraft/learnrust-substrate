mod uutils;

fn main() {}

#[cfg(test)]
mod tests {

    use super::uutils::tools::*;
    use crate::uutils::graph::*;

    #[test]
    fn test_sum_u32() {
        let sum = sum_u32(&vec![2, u32::MAX]);
        assert_eq!(sum, Option::None);
        let sum = sum_u32(&vec![2, 3, 4, 5, 6, 7, 8]);
        assert_eq!(sum.unwrap(), 35);
    }

    #[test]
    fn test_print_area() {
        let t = Triangle::new(3.3, 4.4);
        print_area(&t);
        let c = Circular::new(5.5);
        print_area(&c);
        let s = Square::new(6.6);
        print_area(&s);
    }
}
