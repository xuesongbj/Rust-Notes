fn approx_equal(a: f64, b: f64, decimal_places: u8) -> bool {
    let factor = 10.0f64.powi(decimal_places as i32);
    let a = (a * factor).trunc();
    let b = (b * factor).trunc();
    a == b
}

fn main() {
    let x = 1.2331f64;
    let y = 1.2332f64;
    if !approx_equal(x, y, 4) {
        println!("Success");
    }
}

/*
// clippy check

root@8d75790f92f5:~/rs/ddd/src# cargo clippy
    Finished dev [unoptimized + debuginfo] target(s) in 0.03s
*/