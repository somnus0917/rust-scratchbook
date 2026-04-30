fn main() {
    println!("hello drop");

    let _drop_a = Droppable { name: "a" };
    {
        let _drop_b = Droppable { name: "b" };
        {
            let _drop_c = Droppable { name: "c" };
            let _drop_d = Droppable::new("d");
        }
    }
}

struct Droppable {
    name: &'static str,
}
impl Drop for Droppable {
    fn drop(&mut self) {
        println!("正在释放{}......", self.name)
    }
}

impl Droppable {
    fn new(name: &'static str) -> Droppable {
        Droppable { name }
    }
}
