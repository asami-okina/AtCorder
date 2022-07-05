use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    let sorce = AutoSource::from(
        "
        101
        "
    );

    input!(
        from sorce,
        a: String,
    );

    let mut multiplication_result = 0;

    // 文字列から1文字ずつ取得
    for i in a.as_str().chars(){
        if i == '1' {
            multiplication_result += 1;
        }
    }
    println!("{}",multiplication_result);

}

// 提出コード
// use proconio::input;

// fn main() {
//     input!(
//         a: String,
//     );

//     let mut multiplication_result = 0;

//     for i in a.as_str().chars(){
//         if i == '1' {
//             multiplication_result += 1;
//         }
//     }
//     println!("{}",multiplication_result);

// }
