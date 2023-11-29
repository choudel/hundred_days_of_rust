trait Abstraction {
    fn operation(&self);
}
trait Implementor {
    fn operation_impl(&self);
}
struct ConcreteImplementor;
impl Implementor for ConcreteImplementor {
    fn operation_impl(&self) {
        println!("ConcreteImplementor: operation_impl")
    }
}

struct RefinedAbstraction {
    implementor: Box<dyn Implementor>,
}

impl RefinedAbstraction {
    fn new(implementor: Box<dyn Implementor>) -> RefinedAbstraction {
        RefinedAbstraction { implementor }
    }
}

impl Abstraction for RefinedAbstraction {
    fn operation(&self) {
        self.implementor.operation_impl();
    }
}
fn main() {
    let implementor = ConcreteImplementor;
    let abstraction = RefinedAbstraction::new(Box::new(implementor));
    abstraction.operation()
}
