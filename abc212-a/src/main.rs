use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     100 2
    //     ",
    // );

    input! {
        // from source,
        a: usize,
        b: usize,
    };
    println!(
        "{}",
        if 0 < a && b == 0 {
            "Gold"
        } else if a == 0 && 0 < b {
            "Silver"
        } else {
            "Alloy"
        }
    );
}
