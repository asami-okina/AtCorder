
use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     333
    //     "
    // );

    input! {
        // from source,
        h: f64
    };

    // 数値を0.5乗することで、数値の正の平方根を算出できる
    // (例)√4 = 2 、4^0.5 = 2となり、結果が一致する
    let result = (h * (12800000.0 + h)).powf(0.5);
    println!("{:.9}", result);
}
