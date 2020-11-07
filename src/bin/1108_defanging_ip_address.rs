struct Solution;

impl Solution {
    pub fn defang_i_paddr(address: String) -> String {
        address.replace(".", "[.]")
    }
}

fn main() {}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_defang_i_paddr() {
        assert_eq!(
            Solution::defang_i_paddr(String::from("1.1.1.1")),
            String::from("1[.]1[.]1[.]1")
        );
    }
}
