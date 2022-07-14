use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     7
    //     ooooooo              
    //     "
    // );

    input! {
        // from source,
        n: usize,
        s: String
    };
    let new_s: Vec<char> = s.chars().collect();
    println!("{}", if new_s[n - 1] == 'o' {"Yes"} else {"No"});

}