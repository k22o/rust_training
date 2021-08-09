/**
 * https://doc.rust-jp.rs/book-ja/ch10-03-lifetime-syntax.html 
 * ライフタイム：その参照が有効になるスコープのこと
 * 'x で、ライフタイムを定めることができる
 * 
 * 
 * 
 */


/*
状況に応じて、xとyのどちらを参照するのかわからず、
ライフタイムが定められないので、エラーが生じる。
fn longest1(x: &str, y: &str) -> &str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}
*/

/*
xとyへの共通したライフタイムを設定することで、
上記の問題を解決できる
*/
fn longest2<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        return x;
    } else {
        return y;
    }
}

/*
変数が1つの場合、自動でライフタイムが保管されるので、省略可
return_self <'a>(x :&'a str) -> &'a str
*/
fn return_self(x: & str) -> &str {
        return x;
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    //let result = longest(string1.as_str(), string2);
    let result = longest2(string1.as_str(), string2);
    println!("The longest string is {}", result);
    println!("string1: {}", string1);
    println!("string2: {}", string2);

}

