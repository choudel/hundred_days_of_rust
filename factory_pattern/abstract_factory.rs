use std::fmt::Debug;
trait Product: Debug {
    fn new() -> Self
    where
        Self: Sized;
}
#[derive(Debug)]
struct ProductA;
#[derive(Debug)]
struct ProductB;

impl Product for ProductA {
    fn new() -> Self {
        ProductA
    }
}

impl Product for ProductB {
    fn new() -> Self {
        ProductB
    }
}

trait Factory {
    fn get_product_a(&self) -> Box<dyn Product>;
    fn get_product_b(&self) -> Box<dyn Product>;
}
#[derive(Debug)]
struct ConcreteFactory {
    kind: i32,
}

impl Factory for ConcreteFactory {
    fn get_product_a(&self) -> Box<dyn Product> {
        Box::new(ProductA::new())
    }

    fn get_product_b(&self) -> Box<dyn Product> {
        Box::new(ProductB::new())
    }
}
fn main() {
    let factory = ConcreteFactory { kind: 1 };

    let product_a = factory.get_product_a();
    println!("Created {:?}", product_a);

    let product_b = factory.get_product_b();
    println!("Created {:?}", product_b);
}
