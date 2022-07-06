use proconio::input;
use proconio::source::auto::AutoSource;
use std::collections::HashSet;

fn main() {
    let source = AutoSource::from(
        "
        100 4 16
        ",
    );

    input!(
        from source,
        sum: String,
        a: i32,
        b: i32
    );

    let mut numric_each_digits_vec: Vec<i32> = vec![];
    let mut result_hash_set: HashSet<i32> = HashSet::new();

    for i in sum.as_str().chars() {
        let numric_each_digits = i as i32 - 48;
        numric_each_digits_vec.push(numric_each_digits);
    }

    let sum: i32 = sum.parse().unwrap();

    for i in 0..numric_each_digits_vec.len() {
        for j in 0..numric_each_digits_vec[i] + 1 {
            for k in 0..10 {
                // 桁ごとの数字を合算
                let combined_number_each_digits: i32 = j + k;
                if a <= combined_number_each_digits && combined_number_each_digits <= b {
                    let string_result: String = format!("{}{}", j, k);
                    let num_result: i32 = string_result.parse().unwrap();
                    if num_result <= sum {
                        result_hash_set.insert(num_result);
                    }
                }
            }
        }
    }
    let mut result: i32 = 0;

    for num in result_hash_set {
        result += num;
    }
    println!("{}", result);
}
