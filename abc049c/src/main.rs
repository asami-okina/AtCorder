use proconio::input;
use proconio::source::auto::AutoSource;
use regex::Regex;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     erasedreamer
    //     "
    // );

    input!(
        // from source,
        mut s: String
    );
    // 制約
    // 1<= S <= 10^5(100,000)

    // メモ
    // replaceの順序が大切。
    // dreamerを一番最初に消してしまうと、「dreamerase/dremeraser」の時、erase/eraserに影響してしまう
    let s: String = s.replace("eraser", "");

    let s: String = s.replace("erase", "");

    let s: String = s.replace("dreamer", "");

    let s: String = s.replace("dream", "");

    if s.is_empty() {
        // ""かどうか
        println!("YES");
    } else {
        println!("NO");
    }
}
