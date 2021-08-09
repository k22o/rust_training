/**
 * https://doc.rust-jp.rs/book-ja/ch09-00-error-handling.html
 * 
 * rustでは、error処理をpanic!を用いて行う (標準エラー出力)。
 * error処理には、matchを使う場合と？を使う場合がある。
 * 
 * 
*/

use std::fs::File;
use std::io;
use std::io::Read;

fn main() {

    let swithing_flag = 3;

    if swithing_flag == 1 {

        panic!("panic! panic!");

    } else if swithing_flag == 2{

        let f = File::open("hello.txt");
        /*
        File::openの返り値は、以下のenum Result型である。
        
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
    
        引数は、
        Ok -> std::fs::File
        Err -> std::io::Error
        で与えられる。
        */
    
        let f = match f {
            Ok(file) => file,
            Err(error) => {
                // ファイルを開く際に問題がありました
                panic!("There was a problem opening the file: {:?}", error)
            },
        };
    
    } else if swithing_flag == 3 {

        // ?演算子を関数内で使うと、errorの場合はerror出力をして終了
        // ?演算子は、Resultを返す変数内でしか使うことはできない
        let result = read_from_file();

    }
}

fn read_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}