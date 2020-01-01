extern crate crate_demo;

use crate_demo::chinese::{morning, evening};
use crate_demo::spanish;

fn main() {
    println!("おはよう in Chinese: {}", morning::morning());
    println!("こんばんは in Chinese: {}", evening::evening());

    println!("おはよう in Spanish: {}", spanish::morning());
    println!("こんばんは in Spanish: {}", spanish::evening());
}
