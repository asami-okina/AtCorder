use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     2
    //     2
    //     2
    //     100
    //     ",
    // );

    input!(
        // from source,
        five_hundred_yen_count: i32, // 500円
        one_hundred_yen_count: i32, // 100円
        fifty_yen_count: i32, // 50円
        total_yen: i32
    );

    // 計算回数
    let mut total_calculate_count: i32 = 0;

    // 0枚のパターンもあるので、rangeには+1する
    for five_hundred_yen_index in 0..five_hundred_yen_count + 1 {
        for one_hundred_yen_index in 0..one_hundred_yen_count + 1 {
            for fifty_yen_index in 0..fifty_yen_count + 1 {
                let result: i32 = 500 * five_hundred_yen_index
                    + 100 * one_hundred_yen_index
                    + fifty_yen_index * 50;
                if result == total_yen {
                    total_calculate_count += 1;
                }
            }
        }
    }
    println!("{}", total_calculate_count);
}
