use std::time::Instant;

struct Solution;

impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        // nums.rotate_right(k as usize);

        fn reverse(nums: &mut Vec<i32>, lo: usize, hi: usize) {
            let mut lo = lo;
            let mut hi = hi;
            while lo < hi {
                let tmp = nums[lo];
                nums[lo] = nums[hi];
                nums[hi] = tmp;
                lo += 1;
                hi -= 1;
            }
        }

        if nums.len() > 1 {
            let k = (k as usize) % nums.len();
            let n = nums.len() - (1 as usize);
            reverse(nums, 0, n - k);
            reverse(nums, n - k + 1, n);
            reverse(nums, 0, n);
        }
    }
}

fn main() {
    let now = Instant::now();

    let v = &mut vec![1, 2, 3, 4, 5, 6, 7];
    Solution::rotate(v, 3);
    println!("{:?}", v);

    println!("{}", now.elapsed().as_micros());
}