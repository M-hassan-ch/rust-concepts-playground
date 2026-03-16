#![allow(unused)]

#[derive(Debug)]
struct Cat;
#[derive(Debug)]
struct Dog;
#[derive(Debug)]
struct Lion;

trait Animal {
    fn speak(&self) {
        println!("Animal")
    }
}

impl Animal for Dog {
    fn speak(&self) {
        println!("Woof")
    }
}
impl Animal for Cat {
    fn speak(&self) {
        println!("Meow")
    }
}
impl Animal for Lion {}

fn static_speak(animal: &impl Animal) {
    animal.speak();
}

fn dynamic_speak(animal: &dyn Animal) {
    animal.speak();
}

fn get_dynamic_type(random: i32) -> Box<dyn Animal> {
    if random < 10 {
        Box::new(Cat)
    } else {
        Box::new(Dog)
    }
}

fn main() {
    let cat = Cat;
    let dog = Dog;
    let lion = Lion;

    // static dispatching
    static_speak(&cat);
    static_speak(&dog);
    static_speak(&lion);

    let random = 10;
    let animal: &dyn Animal = if random < 10 { &Cat } else { &Dog };

    // dynamic dispatching
    dynamic_speak(animal);
    let d_type = get_dynamic_type(random);
    d_type.speak();
}
