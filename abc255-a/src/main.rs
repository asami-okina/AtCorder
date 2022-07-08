use proconio::input;
use proconio::source::auto::AutoSource;
fn main() {
    // let source = AutoSource::from(
    //     "
    //     1 2
    //     1 0
    //     0 1
    //     "
    // );

    input!(
        // from source,
        row: usize,
        column: usize,
        list: [i32; 4]
    );

    // [[1,0][0,1]]
    let result:[[i32;2];2] = [[list[0], list[1]],[list[2], list[3]]];
    let selected_row: usize = row - 1;
    let selected_column: usize = column - 1;
    println!("{:?}",result[selected_row][selected_column]);
}
