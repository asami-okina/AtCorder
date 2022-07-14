use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     a aa
    //     "
    // );

    input! {
        // from source,
        s: String,
        t: String
    };

    println!("{}", if s < t { "Yes" } else { "No" });
}
