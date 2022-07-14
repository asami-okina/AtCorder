use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     2 11
    //     "
    // );

    input! {
        // from source,
        a: i32,
        b: i32,
    };

    // A回サイコロを振って得られる最小値、最大値
    let min = a;
    let max = a * 6;

    println!("{}", if b >= min && b <= max {"Yes"} else {"No"});

}