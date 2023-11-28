#[macro_use]
extern crate lazy_static;
use std::sync::Mutex;
pub struct Singleton {
    value: String,
}
impl Singleton {
    pub fn new() -> Singleton {
        Singleton {
            value: String::from("I'm a singleton!"),
        }
    }
}
lazy_static! {
    pub static ref SINGLETON: Mutex<Singleton> = Mutex::new(Singleton::new());
}
fn main() {
    // Lock the Mutex before accessing the Singleton
    {
        let singleton = SINGLETON.lock().unwrap();

        // Now you can access the value in the Singleton
        println!("{}", singleton.value);
    }
    {
        let singleton = SINGLETON.lock().unwrap();
        println!("{}", singleton.value);
    }
}
