use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     9 45000
    //     "
    // );

    input!(
        // from source,
        count: i32,
        total: i32
    );

    for ten in 0..=count {
        // 5000円札は1万円札の枚数を引いた分だけfor分で回す
        for five in 0..=count - ten {
            if ten * 10000 + five * 5000 + (count - ten - five) * 1000 == total {
                println!("{} {} {}", ten, five, count - ten - five);
                return;
            }
        }
    }
    println!("{} {} {}", -1, -1 ,-1);
}
