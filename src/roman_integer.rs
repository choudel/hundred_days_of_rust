use std::collections::HashMap;
fn roman_to_int(s: String) -> i32 {
    let map: HashMap<char, i32> = vec![
        ('I', 1),
        ('V', 5),
        ('X', 10),
        ('L', 50),
        ('C', 100),
        ('D', 500),
        ('M', 1000),
    ]
    .into_iter()
    .collect();
    let mut sum = 0;
    let mut last = 0;
    for c in s.chars() {
        if let Some(&v) = map.get(&c) {
            if v > last {
                sum += v - last - last;
            } else {
                sum += v;
            }
            last = v
        }
    }
    sum
}
fn main() {
    let result = roman_to_int("IX".to_string());
    println!("{}", result);
}
