use proconio::input;
use proconio::source::auto::AutoSource;
 
fn main() {
    // let source = AutoSource::from(
    //     "
    //     0 0
    //     "
    // );

    input! {
        // from source,
        x: i32,
        y: i32,
    };

    // 前提　あいこ
    // 0はグー
    // 1はチョキ
    // 2はパー

    if x == y {
        println!("{}", x);
    }

    let list: Vec<i32> = vec![0,1,2];

    for i in list {
        if i != x && i !=
         y {
            println!("{}", i);
        }
    }
    

}