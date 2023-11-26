fn is_valid(s: String) -> bool {
    let mut stk: Vec<char> = vec![];
    for char in s.chars() {
        match char {
            '(' | '{' | '[' => stk.push(char),
            ')' | '}' | ']' => match stk.pop() {
                Some(t) => {
                    if !((t == '(' && char == ')')
                        || (t == '[' && char == ']')
                        || (t == '{' && char == '}'))
                    {
                        return false;
                    }
                }
                None => {
                    return false;
                }
            },
            _ => {}
        }
    }
    stk.is_empty()
}
fn main() {
    println!("{}", is_valid("{} [] [] ()".to_string()));
}
