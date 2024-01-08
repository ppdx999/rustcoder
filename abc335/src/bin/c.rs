use proconio::input;

fn main() {
    input! { n: usize, q: usize }
    let mut v = vec![];

    for i in (1..=n).rev() {
        v.push((i as isize, 0 as isize));
    }

    for _ in 0..q {
        input! { cmd: usize }
        match cmd {
            1 => {
                input! { c: char }
                let (px, py) = *v.last().unwrap();

                match c {
                    'R' => v.push((px + 1, py)),
                    'L' => v.push((px - 1, py)),
                    'U' => v.push((px, py + 1)),
                    'D' => v.push((px, py - 1)),
                    _ => unreachable!(),
                }
            }
            2 => {
                input! { p: usize }
                let (x, y) = v[v.len() - p];
                println!("{} {}", x, y);
            }
            _ => unreachable!(),
        }
    }
}
