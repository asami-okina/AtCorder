use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     459230781
    //     "
    // );

    input! {
        // from source,
        mut s: String
    };

    let num_list: String = String::from("0123456789");
    let mut result: String = String::new();
    num_list.chars().for_each(|char| 
    {
        if !s.contains(char) {
            result.push(char);
        }
    }
    );
    println!("{}",result);
}
