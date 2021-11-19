
fn main() {
    let strs = vec![String::from("facebook"), String::from("factory"), String::from("facy")];
    let prefix = strs.iter().map(|x| x.len()).min_by(|a, b| a.cmp(b));
    println!("Prefix = {:?}", prefix)
}
