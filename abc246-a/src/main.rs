
use proconio::input;
use proconio::source::auto::AutoSource;

fn main() {
    // let source = AutoSource::from(
    //     "
    //     -1 -1
    //     -1 2
    //     3 2
    //     "
    // );

    input! {
        // from source,
        list: [(isize, isize); 3]
    };

    let x_1: isize = list[0].0;
    let x_2: isize = list[1].0;
    let x_3: isize = list[2].0;
    let x_4: isize;

    let y_1: isize = list[0].1;
    let y_2: isize = list[1].1;
    let y_3: isize = list[2].1;
    let y_4: isize;

    if x_1 == x_2 {
        x_4 = x_3;
    } else if x_2 == x_3 {
        x_4 = x_1;
    } else {
        x_4 = x_2;
    };

    if y_1 == y_2 {
        y_4 = y_3;
    } else if y_2 == y_3 {
        y_4 = y_1;
    } else {
        y_4 = y_2;
    };

    println!("{} {}", x_4,y_4);
}
