use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     3 14 2
    //     ",
    // );

    input! {
        // from source,
        n: i32,
        k: i32,
        mut a: i32,
    };

    // 前提
    // n: 人数
    // k: 配るカードの総枚数
    // a: 人aから配り始める
    // 最後に配るのは、(a + k - 1)の人

    for _ in 1..=k-1 {
        if a < n {
            a = a + 1;
        } else if n == a {
            a = 1;
        }
    }
    println!("{}", a);
}
