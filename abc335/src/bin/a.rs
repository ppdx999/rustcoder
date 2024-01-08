use ::proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut s = s;
    s.pop();
    s.push('4');
    println!("{}", s);
}
