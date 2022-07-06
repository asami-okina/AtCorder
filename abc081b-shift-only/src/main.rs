// use proconio::input;
// use proconio::source::auto::AutoSource;
// fn main() {
//     // デバッグ
//     // let sorce = AutoSource::from(
//     //     "
//     //     6
//     //     382253568 723152896 37802240 379425024 404894720 471526144
//     //     ",
//     // );
//     input!(
//         // from sorce,
//         count: usize,
//         mut list: [usize;count]
//     );

//     // 操作回数
//     let mut operation_count: i32 = 0;;

//     loop {
//         // 全てが偶数かどうか
//         let is_all_even:bool = is_all_even(&count, &list);

//         if !is_all_even {
//             println!("{}", operation_count);
//             break;
//         } else {
//             // 全ての整数を2で割る
//             devide_integer_by_2(&count, &mut list, &mut operation_count);
//         }
//     }
// }


// // 全て偶数かどうか
// fn is_all_even(count: &usize, list: &Vec<usize>) -> bool {
//     let count = count.clone();
//     let mut is_all_even = true;
//     for index in 0..count {
//         if list[index] % 2 == 1 {
//             is_all_even = false
//         }
//     }
//     is_all_even
// }

// // 全ての整数を2で割る
// fn devide_integer_by_2(count: &usize, list: &mut Vec<usize>, operation_count: &mut i32) {
//     let count = count.clone();
//     for index in 0..count {
//         list[index] = list[index] / 2;
//     }
//     *operation_count += 1;
// }


// シフト演算バージョン
// 下一桁に1が来なければ偶数である
use proconio::input;
use proconio::source::auto::AutoSource;

fn main () {
    // デバッグ
    let sorce = AutoSource::from(
        "
        6
        382253568 723152896 37802240 379425024 404894720 471526144
        ",
    );
    input!(
        from sorce,
        count: usize,
        list: [usize;count]
    );
    // bit
    let mut bit: usize = 0;
    // 操作回数
    let mut operation_count: i32 = 0;

    for i in 0..count {
        // OR(|):どちらかが1なら1、どちらも0の場合は0を返す
        // 右端のビットが1であれば奇数
        println!("for bit1:{}",bit);
        println!("for list[i]:{}",list[i]);
        bit = list[i]| bit;
        println!("bit2:{}",bit);
    }

    // AND(&): どちらも1なら1、どちらか片方でも0の場合は0を返す
    while(bit & 1) == 0 {
        // >>: 右ビットシフト(スイッチのON/OFFの状態を右に1個ずらす)
        // 2進数を右にSビット論理シフトすると、2(-S乗)倍すること(2(s乗)で割ること)に相当
        bit = bit >> 1;
        println!("while bit:{}",bit);
        operation_count += 1;
    }
    println! {"{}",operation_count}
}
