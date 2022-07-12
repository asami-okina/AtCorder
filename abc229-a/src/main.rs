use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     .#
    //     #.
    //     ",
    // );

    input! {
        // from source,
        list_a: String,
        list_b: String
    };
    let new_list_a: Vec<char> = list_a.chars().collect();
    let new_list_b: Vec<char> = list_b.chars().collect();

    if new_list_a[0] == '#' && new_list_a[1] == '#'
        || new_list_a[1] == '#' && new_list_b[1] == '#'
        || new_list_b[1] == '#' && new_list_b[0] == '#'
        || new_list_b[0] == '#' && new_list_a[0] == '#'
    {
        println!("Yes");
    } else {
        println!("No");
    }
}