
/*
# rust の使い方
https://doc.rust-lang.org/book/ch01-02-hello-world.html

(1) main:
    あらゆる名前の関数のうち、mainは特別な関数であり、一番最初に実行されることになる。
    Javaでいうとことの、public static void main(String[] args)
    Pythonでいうところの、if __name__ == '__main__'と同様の役割を果たす。

(2) コンパイルと実行
    コンパイル：rustc ***.rs
    実行：./ ***

(3) cargo
    rustのソースコードのビルドシステムをcargoと呼ぶ。
    https://doc.rust-lang.org/book/ch01-03-hello-cargo.html

*/

fn main() {

    println!("Hello, World");

}