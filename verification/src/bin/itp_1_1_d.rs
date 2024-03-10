// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_D

use input::input;

fn main() {
    input! {s:usize};

    let h = s / 3600;
    let m = s % 3600 / 60;
    let s = s % 60;
    println!("{}:{}:{}", h, m, s);
}
