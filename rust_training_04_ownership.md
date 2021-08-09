<rust_training_02_ownership>
https://doc.rust-jp.rs/book-ja/ch04-02-references-and-borrowing.html


# メモリとownership
- 一般に、メモリの解放には、(a) ガベージコレクションによる自動解放, (b) free関数等による手動解放がある。
- rustでは、第3の方法としてownershipがある。


# ownershipの特徴とルール
ここで挙げる特徴は、プリミティブ型には当てはまらない。
プリミティブ型では、ふつうにコピーされる (値渡し)。

## 1つの変数しか1つのownershipを持てない
以下の例では、"aaa"の所有権がbに移り、aは解放される。

```
let a = String::from("aaa");
let b = a;
```

次のようにして、コピーすることはできる。

```
let b = a.clone();
```


## スコープから消える or 関数に渡すとownershipは消失する。
スコープが終わる、あるいは以下のように関数に渡すと、その時点で解放される。

```
let a = String::from("aaa");
func_** (a);
```

## 不変参照はいくらでもできる
参照先が不変である場合、ポインタ渡しのようなことをすれば、解放されない。

```
let a = String::from("aaa");
let b = &a;
let c = &a;
```

## 可変参照は単独で1回までならＯＫ
競合を防ぐため、可変参照は1つまでなら問題ない。
ただし、不変参照が同時に存在してはいけない。

OK

```
let mut a = String::from("aaa");
let b = &mut a;
```

NG
```
let mut a = String::from("aaa");
let b = & a;
let c = &mut a;
```

