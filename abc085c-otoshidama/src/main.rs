use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     2000 20000000
    //     "
    // );

    input!(
        // from source,
        count: i32,
        total: i32
    );

    // ten: 10,000円
    // five: 5,000円
    // one: 1,000円

    for ten in 0..=count {
        // 5000円札は1万円札の枚数を引いた分だけfor分で回す
        for five in 0..=count - ten {
            let one = count - ten - five;
            if ten * 10000 + five * 5000 + one * 1000 == total {
                println!("{} {} {}", ten, five, one);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1, -1);
}