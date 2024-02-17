use::proconio::input;
use std::collections::HashMap;

fn solve(n: usize, memo: &mut HashMap<usize, usize>) -> usize {
    if n < 2 {
        return 0;
    }

    if let Some(&v) = memo.get(&n) {
        return v;
    }

    let ans = solve(n / 2, memo) + solve((n + 1) / 2, memo) + n;
    memo.insert(n, ans);
    ans
}

fn main() {
    input! {
        n: usize,
    }
    let mut memo = HashMap::new();
    println!("{}", solve(n, &mut memo));
}
