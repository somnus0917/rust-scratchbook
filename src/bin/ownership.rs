fn destory_box(c: Box<i32>) {
    println!("正在摧毁一个包含{}的box", c);
}
fn main() {
    let x = 5u32;
    let y = x;
    println!("x:{},y:{}", x, y);
    let b = Box::new(5i32);
    println!("b包含{}", b);
    let a = b;
    // println!("b包含{}", b);
    println!("a包含{}", a);

    let immutable_box = Box::new(5u32);
    println!("immutable:{}", immutable_box);
    // *immutable_box += 4;
    let mut mutable = immutable_box;
    println!("mutable:{}", mutable);
    *mutable += 3;
    println!("mutable:{}", mutable);
}
