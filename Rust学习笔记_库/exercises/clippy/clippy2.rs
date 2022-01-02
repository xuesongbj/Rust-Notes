fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x
    }

    println!("{}", res);
}

/*
// clippy check

root@8d75790f92f5:~/rs/ddd/src# cargo clippy
    Checking ddd v0.1.0 (/root/rs/ddd)
    Finished dev [unoptimized + debuginfo] target(s) in 0.15s
*/