#![allow(unused)]

use std::{cmp::Ordering, fmt::Debug, slice, usize};

fn bubble_sort<T>(vec: &Vec<T>, asc: bool) -> Vec<T>
where
    T: PartialOrd + Copy,
{
    let mut v = vec.clone();
    let mut o = Ordering::Greater;
    if !asc {
        o = Ordering::Less;
    }
    for i in 0..v.len() - 1 {
        for j in 0..v.len() - 1 - i {
            if v[j].partial_cmp(&v[j + 1]).unwrap() == o {
                let t = v[j + 1];
                v[j + 1] = v[j];
                v[j] = t;
            }
        }
    }
    v
}

fn bubble_sort2<T>(slice: &mut [T])
where
    T: PartialOrd,
{
    for i in 0..slice.len() - 1 {
        for j in 0..slice.len() - 1 - i {
            if slice[j] > slice[j + 1] {
                slice.swap(j, j + 1);
            }
        }
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use crate::bubble_sort;
    use std::cmp::Ordering;

    #[test]
    fn t1() {
        let v = vec![3, 5, 45, 5, 86, 91, 123, 32, 1, 56];
        println!("{:?}", v);
        let v: [i32; 10] = v.clone().try_into().unwrap();
        let vr = [3, 5, 45, 5, 86, 91, 123, 32, 1, 56];
        println!("{:?}", v);
        assert_eq!(v, vr);
        let v = v.to_vec();
        let vr = vec![3, 5, 45, 5, 86, 91, 123, 32, 1, 56];
        println!("{:?}", v);
        assert_eq!(v, vr);
    }

    #[test]
    fn t2() {
        let v = vec![3, 5, 45, 5, 86, 91, 123, 32, 1, 56];
        println!("{:?}", v);
        let v = bubble_sort(&v, true);
        println!("{:?}", v);
        assert_eq!(v, vec![1, 3, 5, 5, 32, 45, 56, 86, 91, 123]);
        let v = bubble_sort(&v, false);
        println!("{:?}", v);
        assert_eq!(v, vec![123, 91, 86, 56, 45, 32, 5, 5, 3, 1]);
    }

    #[test]
    fn t3() {
        let s = "h*ll wor#$";

        let bts = s.as_bytes();

        println!("\nfor");
        for i in bts {
            print!(" {}={}", i, *i as char);
        }

        println!("\nfor iter");
        for i in bts.iter() {
            print!(" {}", i);
        }

        println!("\nfor iter enumerate");
        for (i, v) in bts.iter().enumerate() {
            print!(" {}:{}", i, v);
        }
        println!("\n{:?} {:?}", s, bts);

        assert_eq!(s.clone(), s);

        let s = b"h*ll wor#$";

        assert_eq!(s, bts);
    }
}
