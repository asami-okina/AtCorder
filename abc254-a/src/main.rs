use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     254
    //     "
    // );

    input!(
        // from source,
        n: String
    );
    let parse_string = n.chars().collect::<Vec<char>>();
    let result = format!("{}{}", parse_string[1], parse_string[2]);
    println!("{}", result);
}
