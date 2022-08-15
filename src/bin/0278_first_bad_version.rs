struct Solution;

impl Solution {
    pub fn first_bad_version(&self, n: i32) -> i32 {
        let mut l = 1;
        let mut r = n;
        let mut m;

        while l < r {
            m = l + (r - l) / 2;
            if !self.isBadVersion(m) {
                l = m + 1;
            } else {
                r = m;
            }
        }
        l
    }
}

fn main() {}
