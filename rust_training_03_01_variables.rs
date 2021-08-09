/*
//https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html

# rustの変数のimmutable性
rustにおいて、変数はimuutable、すなわち、一度設定したら変更できない、というのがデフォルトである。
let x = 5　とした後に、 x = 6 とはできない。
(注：他の言語においても、プリミティブ型はimutableである。
しかし、上記のような操作時はあらたにメモリを確保し、それにxというラベルを再付与している。
Rustではこうした使い方は認められていない。)

ただし、mutキーワードをつけることで、可変にすることができる。
-> 意図しないデータの上書きを防ぐ仕組み。

また、再度変数を定義することで上書きすること (shadowing) が可能。
ただし、型が同じ場合に限られる。

*/

fn main() {

    let x = 5;
    println!("x:{}",x);
    // x = 6; //これはエラーが出る。


    let mut y = 5;
    println!("x:{}",y);
    y = 6;
    println!("x:{}",y);
    // これならOK


    let z = 5;
    println!("x:{}",z);
    let z = 6;
    println!("x:{}",z);

    //これはだめ。
    //let z = "string";
    //println!("x:{}",z);

}