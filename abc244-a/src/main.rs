use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     5
    //     abcde
    //     "
    // );

    input! {
        // from source,
        n: usize,
        s: String
    };

    let parse_s: Vec<char> = s.chars().collect();
    println!("{}", parse_s[n - 1]);
}

