/**
 * https://doc.rust-jp.rs/book-ja/ch10-02-traits.html
 * 
 * Javaにおけるinterfaceに相当する機能がtrait
 *  - 1つのtraitは、複数の未定義メソッドを保有することができる
 *  - 構造体やEnumにオーバーライドすることができる
 * 
 * trait境界についてはスキップ
 * https://doc.rust-jp.rs/book-ja/ch10-02-traits.html
 * 
 */
 

struct Employee{
    name :String,
    id : i32
}

//traitの定義方法
pub trait Printable{
    fn return_self(&self) -> String;
    fn print_contents(&self,number:i32);
    fn default_print(&self) {
        println!("default print");
    }
}

//traitをEmployeeに実装する(オーバーライド)
impl Printable for Employee {
    
    fn return_self(&self) -> String {
        return format!("name,id ={}, {}",self.name,self.id);
    }

    fn print_contents(&self, num: i32) {
        for loop_num in 1..num {
            println! ("count: {}, name: {}, id: {}",loop_num,self.name, self.id);
        }
    }
}

fn main() {

    let emp1 = Employee{
        name: String::from("takashi"),
        id:1
    };
    let str1 = emp1.return_self();
    println!("str1: {}",str1);
    emp1.print_contents(3);
    emp1.default_print();

}