use proconio::input;
use proconio::source::auto::AutoSource;

use proconio::derive_readable;

#[derive_readable]
struct List {
    t: i64,
    x: i64,
    y: i64
}
fn main() {
    // let source = AutoSource::from(
    //     "
    //     2
    //     4 2 2
    //     6 0 0
    //     ",
    // );

    input!(
        // from source,
        n: usize,
        txy: [List;n],
    );

    // 2次元平面といえば、格子状を思い浮かべる
    // 最短距離は縦+横(x + y)
    // ①現在の地点から目標地点までの最短到着時刻が移動可能な距離か
    // ②移動できる数(t: 1tに対し1マス進める)と最短距離が同じ奇数か偶数かで判定(余分に進んでも、戻ることができる)

    let mut one_flag: bool;
    let mut two_flag: bool;

    let mut old_x: i64 = 0;
    let mut old_y: i64 = 0;
    let mut old_t: i64 = 0;

    for list in txy {
        let t = list.t;
        let x = list.x;
        let y = list.y;

        // 移動可能な距離(前回のt-今回のt ※t: 1tに対し1マス進める)
        let is_movable_distance = (t - old_t).abs();
        // 現在の地点から目標地点までの最短距離
        let shortest_distance = (x - old_x).abs() + (y - old_y).abs();

        // ①最短距離より移動可能時間が小さければ、false
        one_flag = if shortest_distance <= is_movable_distance {
            true
        } else {
            false
        };

        // ②
        // 移動できる距離(t: 1tに対し1マス進める)が偶数(even)か奇数(odd)か
        let movable_distance_odd_number_or_even_number = if t % 2 == 1 { "odd" } else { "even" };
        // 最短距離が(even)か奇数(odd)か
        let shortest_distance_odd_number_or_even_number =
            if (x + y) % 2 == 1 { "odd" } else { "even" };
        if movable_distance_odd_number_or_even_number == shortest_distance_odd_number_or_even_number
        {
            two_flag = true;
        } else {
            two_flag = false;
        }

        // 前の値として格納
        old_x = x;
        old_y = y;
        old_t = t;

        if !one_flag || !two_flag {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
