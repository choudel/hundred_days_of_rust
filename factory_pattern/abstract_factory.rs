use std::fmt::Debug;
trait Product: Debug {
    fn new(kind: i32) -> Self
    where
        Self: Sized;
}
#[derive(Debug)]
struct ProductA {
    kind: i32,
}
#[derive(Debug)]
struct ProductB {
    kind: i32,
}

impl Product for ProductA {
    fn new(kind: i32) -> Self {
        ProductA { kind }
    }
}

impl Product for ProductB {
    fn new(kind: i32) -> Self {
        ProductB { kind }
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
        Box::new(ProductA::new(self.kind))
    }

    fn get_product_b(&self) -> Box<dyn Product> {
        Box::new(ProductB::new(self.kind))
    }
}
fn main() {
    let factory = ConcreteFactory { kind: 1 };
    let factory2 = ConcreteFactory { kind: 2 };
    let product_a = factory.get_product_a();
    println!("Created {:?}", product_a);

    let product_b = factory.get_product_b();
    println!("Created {:?}", product_b);
    let product_a_2 = factory2.get_product_a();
    println!("Created {:?}", product_a_2);

    let product_b_2 = factory2.get_product_b();
    println!("Created {:?}", product_b_2);
}
