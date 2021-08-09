//https://doc.rust-lang.org/book/ch05-01-defining-structs.html
//https://doc.rust-lang.org/book/ch05-02-example-structs.html

/** 
- 構造体の使い方 -> c++ とほぼ同じ。
- emp2のように、継承のように使うこともできる。
-> クラスが存在しないので、structがその代わりとして活躍する
-> 関数は、implによって実装できる！ (メソッド記法)

- Colorのように、３要素以上のタプルも実装可能。
 
*/

struct Employee {
    name: String,
    id: i16,
    email: String,
    dept: String,
}

impl Employee {
    fn print_name(&self) {
        println! ("name: {}",self.name);
    }
}


struct Color(i32,i32,i32);

fn main() {

    let mut emp = Employee {
        name: String::from("aaa"),
        id: 13,
        email: String::from("bbb@example.com"),
        dept: String::from("ccc"),
    };

    println!("email:{}",emp.email);
    emp.email = String::from("ddd@example.com");
    println!("email:{}",emp.email);
    emp.print_name();


    let mut emp2 = Employee{
        name: String::from("AAAA"),
        ..emp
    };

    println!("email:{}",emp2.name);


    let color = Color(20,20,20);
    println!("tuple0:{}",color.0);
}
