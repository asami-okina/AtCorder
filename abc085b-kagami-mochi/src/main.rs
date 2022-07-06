use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashSet;
use std::cmp::Reverse;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     7
    //     50
    //     30
    //     50
    //     100
    //     50
    //     80
    //     30
    //     "
    // );

    input!(
        // from source,
        mochi_count: i32,
        mut list: [i32; mochi_count]
    );
    list.sort_by_key(|&x| Reverse(x));
    let mut hash_set_list: HashSet<i32> = HashSet::new();

    for &i in &list {
        hash_set_list.insert(i);
    }
    println!("{:?}",hash_set_list.len());

}
