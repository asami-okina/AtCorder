use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     126
    //     ",
    // );

    input! {
        // from source,
        n: usize,
    }

    println!(
        "{}",
        if n >= 1 && n <= 125 {
            4
        } else if n >= 126 && n <= 211 {
            6
        } else {
            8
        }
    );
}
