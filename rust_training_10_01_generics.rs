/**
 * https://doc.rust-jp.rs/book-ja/ch10-00-generics.html
 * 
 * <T>, <E> などによって、ジェネリクスを実現
 * 
 * 
*/

struct Point<T,U> {
    x: T,
    y: U,
}

fn return_self<T>(val :T) -> T {
    return val;
}

fn main() {

    let var1 = Point {x:1, y:"a"};
    println!("{}",var1.x);
    println!("{}",var1.y);

    let var2 = 1;
    let var3 = "a";
    println!("{}",var2);
    println!("{}",var3);
}