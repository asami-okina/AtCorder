use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    let source = AutoSource::from(
        "
        3 4
        "
    );
    input!(
        from source,
        a: i32,
        b: i32
    );

    let multiplication_result = a * b;

    if multiplication_result % 2 == 0 {
        // 偶数
        println!("Even");
    } else {
        // 奇数
        println!("Odd");
    }
}


// 提出コード
// use proconio::input;
// fn main() {
//     input!(
//         a: i32,
//         b: i32
//     );

//     let multiplication_result = a * b;

//     if multiplication_result % 2 == 0 {
//         // 偶数
//         println!("Even");
//     } else {
//         // 奇数
//         println!("Odd");
//     }
// }
