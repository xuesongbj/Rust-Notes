pub fn factorial(num: u64) -> u64 {
    // fold(): 通过操作符将每次执行结果值进行叠加
    (1..num + 1).fold(1, |acc, x| acc * x)
}

fn main() {
    println!("{}", factorial(20));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }

    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}