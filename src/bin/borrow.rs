fn eat_i32_box(boxed_i32: Box<i32>) {
    println!("正在销毁i32的box:{}", boxed_i32)
}

fn show_i32_box(borrow_i32: &Box<i32>) {
    println!("正在借用i32的box:{}", borrow_i32)
}
fn main() {
    let b = Box::new(9i32);
    show_i32_box(&b);
    eat_i32_box(b);
    // show_i32_box(&b);
}
