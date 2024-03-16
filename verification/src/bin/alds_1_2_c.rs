// verification-helper: PROBLEM https://onlinejudge.u-aizu.ac.jp/courses/lesson/1/ALDS1/1/ALDS1_2_C

/// https://qiita.com/tanakh/items/0ba42c7ca36cd29d0ac8
macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
    ($($r:tt)*) => {
        let mut s = {
            use std::io::Read;
            let mut s = String::new();
            std::io::stdin().read_to_string(&mut s).unwrap();
            s
        };
        let mut iter = s.split_whitespace();
        input_inner!{iter, $($r)*}
    };
}

macro_rules! input_inner {
    ($iter:expr) => {};
    ($iter:expr, ) => {};

    ($iter:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($iter, $t);
        input_inner!{$iter $($r)*}
    };
}

macro_rules! read_value {
    ($iter:expr, ( $($t:tt),* )) => {
        ( $(read_value!($iter, $t)),* )
    };

    ($iter:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($iter, $t)).collect::<Vec<_>>()
    };

    ($iter:expr, chars) => {
        read_value!($iter, String).chars().collect::<Vec<char>>()
    };

    ($iter:expr, usize1) => {
        read_value!($iter, usize) - 1
    };

    ($iter:expr, $t:ty) => {
        $iter.next().unwrap().parse::<$t>().expect("Parse error")
    };
}

fn main() {
    input! {n: usize,  a: [String; n]};

    let mut a1 = a.clone();
    bubble_sort(&mut a1);
    println!("{}", &a1.join(" "));
    println!("{}", "Stable");

    let mut a2 = a.clone();
    selection_sort(&mut a2);
    println!("{}", &a2.join(" "));
    println!(
        "{}",
        if is_stable(&a1, &a2) {
            "Stable"
        } else {
            "Not stable"
        }
    );
}

fn bubble_sort(a: &mut Vec<String>) {
    for i in 0..a.len() {
        for j in (i + 1..a.len()).rev() {
            if to_usize(&a[j]) < to_usize(&a[j - 1]) {
                a.swap(j, j - 1);
            }
        }
    }
}

fn selection_sort(a: &mut Vec<String>) {
    for i in 0..a.len() {
        let mut idx = i;
        for j in i..a.len() {
            if to_usize(&a[j]) < to_usize(&a[idx]) {
                idx = j;
            }
        }
        a.swap(i, idx);
    }
}

fn is_stable(a: &Vec<String>, b: &Vec<String>) -> bool {
    for (i, v) in a.iter().enumerate() {
        if v != &b[i] {
            return false;
        }
    }
    true
}

fn to_usize(s: &String) -> usize {
    s.chars().nth(1).unwrap() as usize
}
