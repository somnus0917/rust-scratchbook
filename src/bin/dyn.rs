fn main() {
    let rn = 0.63;
    println!("{}", random_animal(rn).noise());
    println!("helloworld")
}

struct Sheep {}
struct Cow {}
trait Animal {
    fn noise(&self) -> &'static str;
}
impl Animal for Sheep {
    fn noise(&self) -> &'static str {
        "mie ~~~"
    }
}
impl Animal for Cow {
    fn noise(&self) -> &'static str {
        "mou ~~~"
    }
}
fn random_animal(rn: f64) -> Box<dyn Animal> {
    if rn > 0.5 {
        Box::new(Sheep {})
    } else {
        Box::new(Cow {})
    }
}
