/*
https://doc.rust-lang.org/book/ch03-03-how-functions-work.html

# rustの関数
fn name(args) -> return_type {};

returnする変数について、returnを使わずとも、
関数の最後の変数を;なしで書けば、それを返り値にしてくれる。
*/


fn main() {

    function1(5, 6);
    println!("func2:{}",function2(5));

}

fn function1(x: i32, y: i32) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function2(x: i32) -> i32 {

    //どちらか
    //return x + 1;
    x + 1
}