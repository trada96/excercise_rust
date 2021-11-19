struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut list_of_brackets = Vec::new();

        for c in s.chars() {
            match c {
                '(' => list_of_brackets.push(')'),
                '[' => list_of_brackets.push(']'),
                '{' => list_of_brackets.push('}'),
                ')' | ']' | '}' if Some(c) != list_of_brackets.pop() => return false,
                _ => {}
            }
        }

        return list_of_brackets.is_empty()
    }
}


fn main() {
    let check = Solution::is_valid("[]".to_string());
    println!("Result = {:?}", check);
}
