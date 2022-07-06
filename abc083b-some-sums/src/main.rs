use proconio::*;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     100 4 16
    //     ",
    // );

    input!(
        // from source,
        sum: i32,
        a: i32,
        b: i32
    );

    // 結果
    let mut sum_result: i32 = 0;

    for i in 0..sum + 1 {
        let sum: i32 = i.to_string().chars().map(|x| x as i32 -48).sum();

        if a <= sum && b >= sum {
            sum_result += i;
        }
    }
    println!("{}",sum_result);

}
