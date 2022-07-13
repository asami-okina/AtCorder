use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     630 940 314
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
        c: i32,
    };

    // 条件
    // A以上B以下であるようなCの約数
    // 約数とは、ある整数を割り切ることが出来る数
    for i in a..=b {
        if i % c == 0 {
            println!("{}", i);
            return;
        }
    }
    println!("{}", -1);
}