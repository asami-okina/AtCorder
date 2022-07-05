use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // デバッグ
    // let sorce = AutoSource::from(
    //     "
    //     6
    //     382253568 723152896 37802240 379425024 404894720 471526144
    //     ",
    // );
    input!(
        // from sorce,
        a: usize,
        mut b: [usize;a]
    );

    // 操作回数
    let mut operation_count: i32 = 0;

    loop {
        // 全てが偶数かどうか
        let mut is_all_even: bool = true;
        for index in 0..a {
            if b[index] % 2 == 1 {
                is_all_even = false;
            }
            b[index] = b[index] / 2;
        }
        if !is_all_even {
            println!("{}", operation_count);
            break;
        } else {
            operation_count += 1;
            continue;
        }
    }
}
