struct Solution;

impl Solution {
    pub fn is_palindrome(x: i32) -> bool {
        let x_str: String = x.to_string();
        
        let revert: String = x_str.chars().rev().collect::<String>();
        
        return x_str == revert;
    }
}

fn main() {
    let number = 121;
    let check = Solution::is_palindrome(number);

    println!("{:?}", check)
}
