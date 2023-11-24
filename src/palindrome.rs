fn is_palindrome(x: i32) -> bool {
    if x < 0 {
        return false;
    }
    let mut reversed = 0;
    let mut number = x;
    while number > 0 {
        reversed = reversed * 10 + number % 10;
        number /= 10;
    }
    x == reversed
}
fn main() {
    println!("{}", is_palindrome(101));
    println!("{}", is_palindrome(60));
}
