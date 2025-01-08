use colored::Colorize;
use m8b::MagicBall;
use std::env;

fn main() {
    let magic_ball = MagicBall::new();

    println!("{} {}", "Question:".blue(), get_the_question().bold());

    println!(
        "{} {}",
        "The magic eight ball says:".blue(),
        magic_ball.shake().bold().purple()
    );
}

fn get_the_question() -> String {
    let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);

    let mut question = String::new();

    for (index, arg) in args.iter().enumerate() {
        match index {
            0 => {}
            1 => {
                question.push_str(arg);
            }
            _ => {
                question.push(' ');
                question.push_str(arg);
            }
        }
    }

    question
}
