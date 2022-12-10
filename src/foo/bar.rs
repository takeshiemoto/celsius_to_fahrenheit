use std::collections::HashMap;

pub fn celsius_to_fahrenheit(c: i32) -> f32 {
    let base = 10.0;
    let result = (c - 32) as f32 / 1.8;
    (result * base).floor() / base
}

// ベクタ Vec<T>
// 文字列
// ハッシュマップ

pub fn run() {
    // 型注釈あり
    let v1: Vec<i32> = Vec::new();

    // 推論されるので普通は不要
    let v2 = vec![1, 2, 3, 4, 5];

    // 要素を追加するにはミュータブルなVecを生成する
    let mut v3 = vec!["John", "Paul"];
    v3.push("George");
    v3.push("Ringo");

    {
        // v4はこのスコープのみ有効
        // スコープを抜けるとドロップされる
        let v4 = vec!["Yanagita", "Imamiya", "Takeda"];
    }

    // ベクタを読む方法は二つある
    // 添字
    // getメソッド
    let v5 = vec!["Ruby", "Java", "Go", "Rust"];
    let third = &v5[2];
    println!("The third element is {}", third);

    match v5.get(2) {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is not third element"),
    }

    // 参照を得るか、Optionを得るか
    // 添字の場合、存在しない場合はパニック
    // getの場合はNoneを返すのでハンドリングできる

    // 走査
    for v in &v5 {
        println!("{}", v)
    }

    // 文字列
    let mut s1 = String::new();

    // 文字列リテラルからStringに変換する
    let data = "initial comment";
    let s2 = data.to_string();

    // 直接
    let s3 = "Initial comment".to_string();

    // String::from
    let s4 = String::from("Initial comment");

    // 追加
    let mut s5 = String::from("Hello");
    s5.push_str(" world");
    println!("{}", s5);

    // ハッシュマップと所有権
    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");
    let mut map = HashMap::new();
    map.insert(field_name, field_value);

    // 所有権がHashMapにmoveしているため利用できない
    // println!("{}", field_name);
    // println!("{}", field_value);

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    match score {
        Some(score) => println!("score is {}", score),
        None => println!("Error"),
    }

    // 更新
    // 同じ値をinsertする
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 20);
    println!("{:?}", scores);

    // キーに値がなかった時のみ挿入
    scores.entry(String::from("Yellow")).or_insert(100);
    scores.entry(String::from("Blue")).or_insert(100);
    println!("{:?}", scores);
}
