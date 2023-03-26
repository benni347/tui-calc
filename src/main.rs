pub mod add;
// pub mod divide;
pub mod multiply;
pub mod substract;

use crate::add::add;
// use crate::divide::divide;
use crate::multiply::multiply;
use crate::substract::substract;

fn main() {
    let nums = [1, 2, 3, 4, 5];
    println!("{}", add(&nums));
    println!("{}", multiply(&nums));
    println!("{}", substract(&nums));
    //     println!("{}", divide(1, 2));
}
