#[derive(Debug)]
enum Message {
    Move {x: i32, y: i32},          // struct{}
    Echo(String),                   // function(){}
    ChangeColor(i32, i32, i32),     // tuple
    Quit,                           // int
}

impl Message {
    fn call(&self) {
        println!("{:?}", &self);
    }
}

fn main() {
    let messages = [
        Message::Move{x: 10, y: 30},
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(255, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}