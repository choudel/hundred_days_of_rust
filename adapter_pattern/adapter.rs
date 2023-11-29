pub trait OldInterface {
    fn old_request(&self);
}

pub struct OldImplementation;
impl OldInterface for OldImplementation {
    fn old_request(&self) {
        println!("Old request");
    }
}

pub trait NewInterface {
    fn new_request(&self);
}
pub struct NewImplementation;
impl NewInterface for NewImplementation {
    fn new_request(&self) {
        println!("New Request");
    }
}

pub struct Adapter {
    adaptee: Box<dyn OldInterface>,
}
impl Adapter {
    fn new(adaptee: Box<dyn OldInterface>) -> Adapter {
        Adapter { adaptee }
    }
}
impl NewInterface for Adapter {
    fn new_request(&self) {
        self.adaptee.old_request()
    }
}
fn main() {
    let old_implementation = OldImplementation;
    let adapter = Adapter::new(Box::new(old_implementation));
    adapter.new_request();
}
