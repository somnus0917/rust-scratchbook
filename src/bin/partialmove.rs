fn main() {
    #[derive(Debug)]
    struct Person {
        name: String,
        age: Box<u8>,
    }
    let p = Person {
        name: String::from("alice"),
        age: Box::new(20),
    };
    let Person { name, ref age } = p;
    println!("name:{}", name);
    println!("age:{}", age);

    println!("person age:{}", p.age)
}
