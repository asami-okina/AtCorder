use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     1111
    //     "
    // );

    input! {
        // from source,
        s: String
    };

    let new_s: Vec<char> = s.chars().collect();

    let mut result: String = String::new();
    result.push('0');
    result.push(new_s[0]);
    result.push(new_s[1]);
    result.push(new_s[2]);
    println!("{}", result);
}
