/*
https://doc.rust-lang.org/book/ch03-02-data-types.html

# rustのデータ型
変数を利用するときは、letを使用すればよいが、コンパイル時には型変換を行う。
多くの候補が存在する場合には、ソースコードで型をアノテーションしなければ動かない場合がある。

データ型は、
・Integers
・floatting-point
・boolean
・characters
がある。
IntegerやFloatでは、ビット数も指定し、i32, f32などど記述する必要がある。

また、プリミティブなcompoundとして、tupleとarrayが存在する。

*/

fn main() {

    let x: f32 = 5.0 * 0.3;
    println!("x:{}",x);

    let y: bool = true;
    println!("x:{}",y);

    let z: char = 'z';
    println!("x:{}",z);

    let t: (i32, f64, u8) = (500,6.4,1);
    let (t1,t2,t3) = t;
    println!("t1:{0}, {1}",t1,t.0);
    println!("t2:{}, {}",t1,t.1);
    println!("t3:{}, {}",t1,t.2);

    let a1 = [1,2,3,4,5];
    let a2: [i32;5]  = [1,2,3,4,5];
    let a3 =  [3;5];
    println!("a3[2]:{}",a3[2]);

}