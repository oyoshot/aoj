// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/2/ITP1/1/ITP1_1_C

use input::input;

fn main() {
    input! {a: usize, b:usize};
    println!("{} {}", a * b, a * 2 + b * 2);
}
