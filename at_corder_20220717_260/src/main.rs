use proconio::input;
use proconio::marker::{Bytes, Chars};
use proconio::source::auto::AutoSource;
use std::cmp::Reverse;
use std::collections::{BTreeMap, BTreeSet, HashMap};

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Student {
    exmaniee_number: i32,
    math: i32,
    english: i32,
    math_and_english: i32,
}

impl Student {
    pub fn new(exmaniee_number: i32, math: i32, english: i32) -> Self {
        let math_and_english: i32 = math + english;
        Student {
            exmaniee_number,
            math,
            english,
            math_and_english,
        }
    }
}

fn main() {
    // let source = AutoSource::from(
    //     "
    //     6 1 0 2
    //     80 60 80 60 70 70
    //     40 20 50 90 90 80    
    //     ",
    // );

    // 受験者合計6名(n)
    // ①数学の点が高い人から1名(x)
    // ②この時点でまだ合格となっていない人のうち、英語の点が高い人から0名(y)
    // ③この時点でまだ合格となっていない人のうち、数学と英語の合計点が高い方から2名(z)
    // ④ここまでで合格となっていない人は不合格とする
    // ⑤ただし、①〜③までのどの段階でも、同点なら受験番号の小さい方を優先する
    // 合格となった受験生の番号を小さい順に改行区切りで出力せよ

    input! {
        // from source,
        n: i32, 
        x: i32,
        y: i32,
        z: i32,
        math_list: [i32; n],
        english_list: [i32; n],
    };

    // 合格者リスト
    let mut successful_candidate_list: Vec<i32> = Vec::new();

    // 受験者リスト
    let mut all_student_list: Vec<Student> = Vec::new();
    for i in 0..n {
        all_student_list.push(Student::new(
            i + 1,
            math_list[i as usize],
            english_list[i as usize],
        ));
    }

    // sort student by math(1個目はmath,2個目は受験番号)
    // 降順
    all_student_list.sort_by(|a, b| {
        match b.math.cmp(&a.math) {
            // 昇順
            std::cmp::Ordering::Equal => a.exmaniee_number.cmp(&b.exmaniee_number),
            other => other,
        }
    });

    // 数学で選ばれた人のindex
    let mut remove_math_index: Vec<i32> = Vec::new();
    for i in 0..x {
        successful_candidate_list.push(all_student_list[i as usize].exmaniee_number);
        remove_math_index.push(i);
    }

    // 数学で選ばれた人を削除
    for _ in remove_math_index {
        all_student_list.remove(0);
    }

    // 英語で選ばれた人のindex
    let mut remove_english_index: Vec<i32> = Vec::new();

    // sort student by math(1個目はenglish,2個目は受験番号)
    // 降順
    all_student_list.sort_by(|a, b| {
        match b.english.cmp(&a.english) {
            // 昇順
            std::cmp::Ordering::Equal => a.exmaniee_number.cmp(&b.exmaniee_number),
            other => other,
        }
    });

    for i in 0..y {
        successful_candidate_list.push(all_student_list[i as usize].exmaniee_number);
        remove_english_index.push(i);
    }

    // 英語で選ばれた人を削除
    for _ in remove_english_index {
        all_student_list.remove(0);
    }

    // sort student by sum(1個目はsum,2個目は受験番号)
    // 降順
    all_student_list.sort_by(|a, b| {
        match b.math_and_english.cmp(&a.math_and_english) {
            // 昇順
            std::cmp::Ordering::Equal => a.exmaniee_number.cmp(&b.exmaniee_number),
            other => other,
        }
    });

    for i in 0..z {
        successful_candidate_list.push(all_student_list[i as usize].exmaniee_number);
    }

    successful_candidate_list.sort_by(|a, b| a.cmp(&b));

    // 最終出力
    for i in successful_candidate_list {
        println!("{}", i);
    }
}
