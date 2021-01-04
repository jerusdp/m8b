use m8b::MagicBall;

fn main() {
    let magic_ball = MagicBall::new();

    println!("The magic eight ball says: {}", magic_ball.shake());
}
