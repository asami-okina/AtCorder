use proconio::input;
use proconio::source::auto::AutoSource;
use std::cmp::Reverse;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     4
    //     20 18 2 18
    //     ",
    // );

    input!(
        // from source,
        count: i32,
        mut list: [i32;count]
    );

    // listのソート
    list.sort_by_key(|&x| Reverse(x));

    // 結果
    let mut result: i32 = 0;

    for (index, num) in list.iter().enumerate() {
        if index % 2 == 0 {
            result += num;
        } else {
            result -= num;
        }
    }

    println!("{}", result);
}
