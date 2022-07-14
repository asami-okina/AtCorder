use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     Hello!World!
    //     "
    // );

    input! {
        // from source,
        s: String
    };

    let base: String = String::from("Hello,World!");

    println!("{}", if base == s {"AC"} else {"WA"});
}