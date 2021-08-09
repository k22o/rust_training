/**
 * Collectionの詳細は
 * https://doc.rust-lang.org/std/collections/index.html
 * 
 * (1) Vector
 * https://doc.rust-jp.rs/book-ja/ch08-01-vectors.html
 * 配列とVectorは異なる！
 * 値の取得方法が2つある！
 * 
 * 
 * (2) String
 * https://doc.rust-jp.rs/book-ja/ch08-02-strings.html
 * 通常のstringに加えて、ライブラリとして提供されるStringがある。
 * Stringは、可変だけど、各要素にアクセスできなかったり、かなり複雑！
 * 
 * 
 * (3) Hash Map
 * https://doc.rust-jp.rs/book-ja/ch08-03-hash-maps.html 
 * 
 * 
 */

use std::collections::HashMap;

fn main() {

    let mut v1: Vec<i32> = Vec::new();
    v1.push(1);
    v1.push(2);
    v1.push(3);

    let v2 = vec![1,2,3];

    //2つのアクセス方法
    let first1: &i32 = &v2[0];
    let first2: Option<&i32> = v2.get(0);

    println!("first1:{}, first2:{}",first1,first2.unwrap());
    for i in &v2 {
        println!("{}",i);
    }

    ////////////////////////////////////////////////////////

    let mut s1 = String::new();
    let s2 = "test2".to_string();
    let s3 = String::from("test3");
    s1.push_str("test1");
    println!("s1:{}",&s1);
    println!("s2:{}",&s2);
    println!("s3:{}",&s3);

    let s4 = s1 + " " + &s2;
    let s5 = format!("{} {}", &s2,&s3);

    println!("s4:{}",&s4);
    println!("s5:{}",&s5);

    ////////////////////////////////////////////////////////

    let mut scores = HashMap::new();
    scores.insert(String::from("blue"),10);
    scores.insert(String::from("red"),20);
    let name1 = String::from("blue");
    let score1 = scores.get(&name1);
    println!("name:{}, score:{}",&name1,&score1.unwrap());

    for(key,value) in &scores {
        println!("name:{}, score:{}",key,value);
    }

}