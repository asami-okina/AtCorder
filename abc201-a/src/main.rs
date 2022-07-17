use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     5 5 5
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };
    // 等差数列とは、はじめの数に一定の数を足し続ける数列

    let mut list: Vec<i32> = vec![a, b, c];
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());

    if list[2] - list[1] == list[1] - list[0] {
        println!("Yes");
    } else {
        println!("No");
    }
}
