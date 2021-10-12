use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError>{
    let mut tokens = 100;
    let pretend_user_input = "8";
    let cost = total_cost(pretend_user_input)?;     // `?`运算符只能用在返回`Result` 或 `Option` 的函数中

    if cost > tokens {
        println!("You can't afford that many!");
    } else {
        tokens -= cost;
        println!("You now have {} tokens.", tokens);
    }

    Ok(())
}

pub fn total_cost(item_quantity: &str) -> Result<i32, ParseIntError> {
    let processing_fee = 1;
    let cost_per_item = 5;
    let qty = item_quantity.parse::<i32>()?;

    Ok(qty * cost_per_item + processing_fee)
}