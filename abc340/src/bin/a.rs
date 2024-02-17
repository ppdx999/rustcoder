use ::proconio::input;

fn main() {
    input! {
        a: i32,
        b: i32,
        c: i32,
    }
    let mut i = a;
    loop {
        print!("{}", i);
        i += c;
        if i > b {
            break;
        }
        print!(" ");
    }
}
