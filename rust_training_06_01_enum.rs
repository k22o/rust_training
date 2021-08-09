/**
 * https://doc.rust-jp.rs/book-ja/ch06-01-defining-an-enum.html
 * 
 * 列挙型Enum と　Match演算子
 * 
 * - enumの列挙子に直接値を格納することができる
 * - implによって、関数も実装できる (一般言語のクラスの代用機能)
 * - match演算子によって、パターンごとの値を返すことができる
 * 
 * Optionalの内容については割愛
 * 
 * if - let の内容については割愛
 * https://doc.rust-jp.rs/book-ja/ch06-03-if-let.html
 * 
 */

//Enumの定義方法
enum YearEnum {
    JAN,
    FEB,
    MAR
}

//変数を有するEnumの定義方法
enum YearEnum2 {
    JAN(u16),
    FEB(u16),
    MAR(u16)
}

//Enum型へのメソッドの実装
impl YearEnum2 {
    fn print_number(&self) {
        println!("abcde");
    }
}

//match文によるフロー制御１
fn change_number(year: YearEnum) -> u16 {
    match year {
        YearEnum:: JAN => 1,
        YearEnum:: FEB => 2,
        YearEnum:: MAR => {
            println! ("Mar:3");
            3
        },
    }
}

//match文によるフロー制御2
fn num2str(num :u16) {
    match num{
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => (),
    }
}


fn main() {

    let year1 = YearEnum::JAN;//Enumの定義
    let year2 = YearEnum2::FEB(2);//変数を有するEnumの定義
    year2.print_number();//Enum型への関数の実装
    let year3 = change_number(YearEnum::MAR);//matchによるフロー制御
    num2str(1);
    num2str(10);
}

