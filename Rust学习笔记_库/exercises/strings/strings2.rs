fn main() {
    let word = String::from("green");
    
    // 参数是&str类型
    if is_a_color_word(&word) {
        println!("That is a color word I know.");
    } else {
        println!("That is not a color word I know.");
    }
}

fn is_a_color_word(attempt: &str) -> bool {
    attempt == "green" || attempt == "blue" || attempt == "red"
}