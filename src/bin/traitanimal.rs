fn main() {
    // 在这种情况下需要类型标注
    let mut dolly = Sheep::new("多莉");

    dolly.talk();
    dolly.shear();
    dolly.talk();
    dolly.shear();
    dolly.talk();
}

struct Sheep {
    naked: bool,
    name: &'static str,
}
trait Animal {
    fn new(name: &'static str) -> Self;

    fn name(&self) -> &'static str;
    fn noise(&self) -> &'static str;

    fn talk(&self) {
        println!("{} 说了{}", self.name(), self.noise())
    }
}
impl Sheep {
    fn is_naked(&self) -> bool {
        self.naked
    }
    fn shear(&mut self) {
        if self.is_naked() {
            println!("{}羊已经剃完毛了", self.name)
        } else {
            println!("{} 剃了个毛！", self.name())
        }
        self.naked = true
    }
}
impl Animal for Sheep {
    fn new(name: &'static str) -> Self {
        Sheep {
            naked: false,
            name: name,
        }
    }
    fn name(&self) -> &'static str {
        self.name
    }
    fn noise(&self) -> &'static str {
        if self.is_naked() {
            "mie ???"
        } else {
            "mie !!!"
        }
    }
    fn talk(&self) {
        println!("{}........{}", self.name, self.noise())
    }
}
