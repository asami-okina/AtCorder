
use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     100 100 100
    //     "
    // );

    input!(
        // from source,
        list: [i32;3]
    );

    let mut new_list: Vec<i32> = list.clone();
    new_list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    
    if list[1] == new_list[1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
