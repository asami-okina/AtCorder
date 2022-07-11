use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     aaaabbbb
    //     1 8
    //     "
    // );

    input! {
        // from source,
        s: String,
        a: usize,
        b: usize,
    };

    // 操作用
    let mut operate_s: Vec<char> = s.chars().collect();

    // 比較用
    let compare_s: Vec<char> = s.chars().collect();
    operate_s[a - 1] = compare_s[b - 1];
    operate_s[b - 1] = compare_s[a - 1];
    
    let result: String = operate_s.iter().collect();
    println!("{}",result);
}
