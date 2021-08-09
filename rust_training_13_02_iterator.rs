/**
 * https://doc.rust-jp.rs/book-ja/ch13-02-iterators.html
 * https://doc.rust-lang.org/std/iter/trait.Iterator.html
 * 
 * イテレータ
 * traitとして実装されている。
 * map関数などは、イテレータに対して適用される
 * 
 * 
 */


fn main() {
    
    let v1 = vec![0,1,2,3];

    for val in v1.iter() {
        println!("iter: {}",val);
    }
    assert_eq!(v1.iter().next(),Some(&0));


    //filter関数、map関数などは、イテレータに対して適用する
    //iteratorで返ってくるので、collect()によって、collection型のどれかに変換
    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect();    
    for val in v2.iter() {
        println!("iter: {}",val);
    }

}