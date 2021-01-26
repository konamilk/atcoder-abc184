use proconio::input;

fn main() {
    input!{
        r1: i64,
        c1: i64,
        r2: i64,
        c2: i64,
    }

    let mut x = (r2 - r1);
    let mut y = (c2 - c1);

    let mut ans = 0i64;

    loop {
        if x==0 && y ==0 {
            break;
        }
        ans += 1;

        if x.abs() + y.abs() <= 3 {
            break;
        }

        let xp = x;
        let yp = y;

        if (x > 0 && y > 0) ||  (x < 0 && y < 0){
            x = xp - (xp + yp) / 2;
            y = yp - (xp + yp) / 2;
        }
        else {
            x = xp - (xp - yp) / 2;
            y = yp + (xp - yp) / 2;
        }
    }

    println!("{}", ans)
}

