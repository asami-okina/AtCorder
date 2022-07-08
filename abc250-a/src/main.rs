
use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     3 4
    //     3 4
    //     "
    // );

    input! {
        // from source,
        height: i32, // 外枠の縦
        width: i32, // 外枠の横
        row: i32, // 枠内のrow
        column: i32, // 枠内のcolumn
    };

    let mut count: i32 = 0;

    // 上が存在するか
    if row != 1 {
        count += 1;
    }

    // 下が存在するか
    if row != height {
        count += 1;
    }

    // 左が存在するか
    if column != 1 {
        count += 1;
    }

    // 右が存在するか
    if column != width {
        count += 1;
    }
    println!("{}", count);
}
