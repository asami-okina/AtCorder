use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     20 7 12
    //     ",
    // );

    input! {
        // from source,
        s: i32,
        t: i32,
        x: i32,
    };

    // 条件
    // 日付をまたがない場合、S < T
    // 日付をまたぐ場合、S > T
    // 前日のS時から当日のT時、当日のS時から、翌日のT時に電気がついている = 当日のT時から当日のS時は電気がついていない

    // 日付をまたがない場合
    if s < t {
        // T時には消灯してしまうため、T時は含まない
        if s <= x && x < t {
            println!("Yes");
        } else {
            println!("No");
        }
    }

    // 日付をまたぐ場合
    if s > t {
        // S時には消灯してしまうため、S時は含まない
        if t <= x && x < s {
            println!("No");
        } else {
            println!("Yes");
        }
    }
}
