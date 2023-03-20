#![allow(dead_code)]

pub fn sum_u32(nums: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;
    for v in nums {
        match sum.checked_add(*v) {
            Some(n) => sum = n,
            None => return None,
        }
    }
    Some(sum)
}
