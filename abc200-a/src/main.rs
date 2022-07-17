use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     200
    //     "
    // );

    input! {
        // from source,
        n: i32,
    };

    // 年から世紀を求める場合は、1を引いて100で割り、1を足す
    println!("{}", (n - 1) / 100 + 1);

}