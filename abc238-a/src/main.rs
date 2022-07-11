use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     623947744
    //     ",
    // );

    input! {
        // from source,
        n: u32
    };

    // 前提: n^2より2^nのほうが圧倒的に大きくなる
    // 条件: 2^n > n^2
    // n = 1の場合 2 > 1
    // n = 2の場合 4 = 4 (No)
    // n = 3の場合 8 < 9 (No)
    // n = 4の場合 16 = 16(No)
    // n = 5の場合 32 > 25
    // n = 6の場合 64 > 36
    // n = 7の場合 128 > 49

    if n == 2 || n == 3 || n == 4 {
        println!("No");
    } else {
        println!("Yes");
    }
}
