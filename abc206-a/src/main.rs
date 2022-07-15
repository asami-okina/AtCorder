use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     191
    //     ",
    // );

    input! {
        // from source,
        n: f64,
    };

    let f64_result: f64 = n * 1.08;
    let i64_result = f64_result.round() as i64;

    println!(
        "{}",
        if i64_result < 206 {
            "Yay!"
        } else if i64_result == 206 {
            "so-so"
        } else {
            ":("
        }
    )
}
