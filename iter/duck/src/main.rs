trait Animal {
    fn cry(&self);
}

struct Dog;
struct Cat;

impl Animal for Dog {
    fn cry(&self) {
        println!("bowbow");
    }
}

impl Animal for Cat {
    fn cry(&self) {
        println!("nyaaan");
    }
}

fn cry(animal: Box<dyn Animal>) {
    animal.cry();
}

fn main() {
    let dog = Dog {};
    let cat = Cat {};

    dog.cry();
    cat.cry();
    cry(Box::new(dog));
}

