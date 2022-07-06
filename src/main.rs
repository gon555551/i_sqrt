fn main() {
    let n: i32 = -36;
    println!("sqrt of {} is {}", n as f64, i_sqrt(n as f64));
}

fn i_sqrt(n: f64) -> String {
    let mut u: f64 = n;

    if u * u == u {
        format!("{}", u)
    } else {
        let neg: bool;
        let mut r: f64 = if u > 0.0 {
            neg = false;
            u / 2.0
        } else {
            neg = true;
            u = -u;
            u / 2.0
        };
        let mut t: f64 = u;
        for _ in 0..1000 {
            if r * r == u {
                break;
            } else if r * r > u {
                if t > r {
                    t = r;
                    r = r / 2.0;
                } else {
                    r = (t + r) / 2.0;
                }
            } else {
                if t < r {
                    t = r;
                    r = (r + u) / 2.0;
                } else {
                    r = (r + t) / 2.0;
                }
            }
        }
        match neg {
            true => format!("{}", r) + "i",
            false => format!("{}", r),
        }
    }
}
