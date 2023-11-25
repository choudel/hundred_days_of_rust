fn is_palindrome(val: i32) -> bool {
    if val < 0 {
        return false;
    }
    let mut reversed = 0;
    let mut number = val;
    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }
    reversed == val
}
fn main() {
    println!("{}", is_palindrome(101));
    println!("{}", is_palindrome(60));
}
