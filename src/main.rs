use rand::Rng;
struct Sheep {}
struct Cow {}
struct Horse {}
struct Cat {}

trait Animal {
    // Instance method signature
    fn noise(&self) -> &'static str;
}

// Implement the `Animal` trait for `Sheep`.
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "baaaaah!"
    }
}

// Implement the `Animal` trait for `Cow`.
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "moooooo!"
    }
}

// Implement the `Animal` trait for `Horse`.
impl Animal for Horse {
    fn noise(&self) -> &'static str {
        "hiiiaaa!"
    }
}

// Implement the `Animal` trait for `Cat`.
impl Animal for Cat {
    fn noise(&self) -> &'static str {
        "miau!"
    }
}

// Returns some struct that implements Animal, but we don't know which one at compile time.
fn random_animal(random_number: f64) -> Box<dyn Animal> {
    match random_number {
        x if x < 1.0/4.0 => Box::new(Sheep {}),
        x if x >= 1.0/4.0 && x < 2.0/4.0 => Box::new(Cow {}),
        x if x >= 2.0/4.0 && x < 3.0/4.0 => Box::new(Horse {}),
        x if x >= 3.0/4.0 => Box::new(Cat {}),
        _ => unreachable!(),
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let random_number = rng.gen();
    let animal = random_animal(random_number);
    println!("You've randomly chosen an animal, and it says {}", animal.noise());
}
