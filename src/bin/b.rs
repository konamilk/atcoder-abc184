use proconio::input;
use proconio::marker::Chars;

fn main() {
    input!{
        n: i32,
        x: i32,
        s: Chars
    }

    let mut ans = x;
    for chara in s.into_iter(){
        if chara == 'o' {
            ans += 1
        }
        else {
            ans = std::cmp::max(ans-1, 0)
        }
    }

    println!("{}", ans)
}
