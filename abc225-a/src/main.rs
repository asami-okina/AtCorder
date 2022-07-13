use itertools::Itertools;
use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashSet;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     xyz
    //     "
    // );

    input! {
        // from source,
        s: String
    };
    let new_s: Vec<char> = s.chars().collect();
    // HashSetは重複を許さない
    let mut result: HashSet<String> = HashSet::new();

    for list in (0..3).permutations(3) {
        result.insert(format!(
            "{}{}{}",
            new_s[list[0]], new_s[list[1]], new_s[list[2]]
        ));
    }
    println!("{}", result.len());
}
