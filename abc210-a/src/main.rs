use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     5 3 20 15
    //     "
    // );

    input! {
        // from source,
        mut n: usize,
        a: usize,
        x: usize,
        y: usize
    };

    // 条件
    // キャベツ1個X円
    // キャベツをA個よりも多く買う場合、A+1個以降に買うキャベツは1個Y円
    // キャベツをN個買うために必要な金額

    if n > a {
    // y円で変える個数
    let y_en_count = n - a;

    // x円で変える個数
    let x_en_count = n - y_en_count;

    println!("{}", y_en_count * y + x_en_count * x);
    } else {
        println!("{}", n * x);
    }
}
