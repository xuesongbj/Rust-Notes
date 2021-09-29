trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    fn append_bar(self) -> Self {
        let mut s:Self = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
#[allow(non_snake_case)]
mod tests {
    use super::*;

    #[test]
    fn is_FooBar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_BarBar() {
        assert_eq! (
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}