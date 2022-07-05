use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    let source = AutoSource::from(
        "
        1
        2 3
        test
        "
    );

    input! {
        from source,
        a: i32,
        b: i32,
        c: i32,
        s: String
    };
    println!("{} {}", a + b + c, s);
}

// 提出コード
// use proconio::input;
 
// fn main() {
//     input! {
//         a: i32,
//         b: i32,
//         c: i32,
//         s: String
//     };
//     println!("{} {}", a + b + c, s);
// }