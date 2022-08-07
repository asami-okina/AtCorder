use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashMap;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     12 12 11 1 2
    //     "
    // );

    input! {
        // from source,
        list: [i32; 5]
    };

    let mut hash_map: HashMap<i32,i32> = HashMap::new();

    for int in list {
        *hash_map.entry(int).or_insert(0) += 1;
    }

    let mut two_count: bool = false;
    let mut three_count: bool = false;

    for (_, value) in hash_map {
        if value == 2 {
            two_count = true;
        }
        if value == 3 {
            three_count = true;
        }
    }

    if two_count && three_count {
        println!("Yes");
    } else {
        println!("No");
    }



}