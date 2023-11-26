fn longest_prefix(strs: Vec<String>) -> String {
    if strs.is_empty() {
        return "".to_string();
    }
    let vec_of_chars: Vec<Vec<char>> = strs.iter().map(|s| s.chars().collect()).collect();
    let mut prefix: Vec<char> = vec![];
    let n = vec_of_chars.iter().map(|chars| chars.len()).min().unwrap();
    for i in 0..n {
        let c = vec_of_chars[0][i];
        if vec_of_chars.iter().all(|char| char[i] == c) {
            prefix.push(c)
        } else {
            break;
        }
    }
    prefix.iter().collect()
}
fn main() {
    let veky = vec!["hello".to_string(), "hell".to_string(), "hel".to_string()];
    println!("{}", longest_prefix(veky))
}
