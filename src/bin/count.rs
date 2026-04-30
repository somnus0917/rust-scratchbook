fn main() {
    println!("hellow count ");
    println!("Foo + Bar = {:?}", Foo + Bar);
    println!("Bar + Foo = {:?}", Bar + Foo);
}
struct Foo;
struct Bar;

#[derive(Debug)]
struct FooBar;
#[derive(Debug)]
struct BarFoo;
use std::ops;
impl ops::Add<Foo> for Bar {
    type Output = BarFoo;
    fn add(self, rhs: Foo) -> Self::Output {
        BarFoo
    }
}

impl ops::Add<Bar> for Foo {
    type Output = FooBar;
    fn add(self, rhs: Bar) -> Self::Output {
        FooBar
    }
}
