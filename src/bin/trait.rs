fn main() {
    println!("Hello, world!");
    let d = Dog;
    play_sound(&d);
    let c = Cat;
    play_sound(&c);
}

pub trait MakeSound {
    fn sound(&self) -> String;
}
struct Dog;
struct Cat;
impl MakeSound for Cat {
    fn sound(&self) -> String {
        String::from("miaomiao")
    }
}
impl MakeSound for Dog {
    fn sound(&self) -> String {
        String::from("wangwang")
    }
}
fn play_sound(animal: &impl MakeSound) {
    println!("发出声音：{}", animal.sound())
}
