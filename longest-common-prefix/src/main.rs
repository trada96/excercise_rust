struct Solution;

impl Solution {
    pub fn longest_common_prefix(strs: Vec<String>) -> String {
        
        let min_length = strs.iter().map(|x| x.len()).min().unwrap();

        for i in (1..min_length+1).rev() {
            let prefix = &strs[0][0..i];
            if strs.iter().all(|x| x.find(prefix) == Some(0)){
                return prefix.to_string();
            }

            }

        String::new()
        }
}

fn main() {
    let strs = vec![String::from("facebook"), String::from("factory"), String::from("facy")];
    let prefix = Solution::longest_common_prefix(strs);
    println!("Prefix = {}", prefix)
}
