use ::proconio::input;

fn main() {
    input! {
        q: i32,
    }

    let mut arr = vec![];

    for _ in 0..q {
        input! {
            cmd: usize,
        }

        match cmd {
            1 => {
                input! {
                    x: i32,
                }
                arr.push(x);
            }
            2 => {
                input! {
                    k: usize,
                }
                println!("{}", arr[arr.len() - k]);
            }
            _ => unreachable!(),
        }
    }
}
