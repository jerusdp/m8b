use rand::seq::SliceRandom;

use rand::Rng;
use std::fmt;

#[derive(Debug)]
enum Responses {
    AsSeen,
    Later,
    BetterNotNow,
    CannotPredict4,
    Concentrate,
    DoNotCount,
    Certain,
    Decidedly,
    MostLikely,
    ReplyNo,
    SourcesNo,
    OutlookNotGood,
    OutlookGood,
    ReplyHazy,
    SignsYes,
    VeryDoubtful,
    WithoutDoubt,
    Yes,
    YesDefinite,
    RelyOnIt,
}

impl fmt::Display for Responses {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Responses::AsSeen => write!(f, "As I see it, yes."),
            Responses::Later => write!(f, "Ask again later."),
            Responses::BetterNotNow => write!(f, "Better not tell you now."),
            Responses::CannotPredict4 => write!(f, "Cannot predict now."),
            Responses::Concentrate => write!(f, "Concentrate and ask again."),
            Responses::DoNotCount => write!(f, "Don’t count on it."),
            Responses::Certain => write!(f, "It is certain."),
            Responses::Decidedly => write!(f, "It is decidedly so."),
            Responses::MostLikely => write!(f, "Most likely."),
            Responses::ReplyNo => write!(f, "My reply is no."),
            Responses::SourcesNo => write!(f, "My sources say no."),
            Responses::OutlookNotGood => write!(f, "Outlook not so good."),
            Responses::OutlookGood => write!(f, "Outlook good."),
            Responses::ReplyHazy => write!(f, "Reply hazy, try again."),
            Responses::SignsYes => write!(f, "Signs point to yes."),
            Responses::VeryDoubtful => write!(f, "Very doubtful."),
            Responses::WithoutDoubt => write!(f, "Without a doubt."),
            Responses::Yes => write!(f, "Yes."),
            Responses::YesDefinite => write!(f, "Yes – definitely."),
            Responses::RelyOnIt => write!(f, "You may rely on it."),
        }
    }
}

#[derive(Debug)]
pub struct MagicBall {
    ball: [Responses; 20],
}

impl MagicBall {
    pub fn new() -> MagicBall {
        let mut ball = [
            Responses::AsSeen,
            Responses::Later,
            Responses::BetterNotNow,
            Responses::CannotPredict4,
            Responses::Concentrate,
            Responses::DoNotCount,
            Responses::Certain,
            Responses::Decidedly,
            Responses::MostLikely,
            Responses::ReplyNo,
            Responses::SourcesNo,
            Responses::OutlookNotGood,
            Responses::OutlookGood,
            Responses::ReplyHazy,
            Responses::SignsYes,
            Responses::VeryDoubtful,
            Responses::WithoutDoubt,
            Responses::Yes,
            Responses::YesDefinite,
            Responses::RelyOnIt,
        ];
        let mut rng = rand::rng();
        ball.shuffle(&mut rng);
        MagicBall { ball }
    }

    pub fn shake(&self) -> String {
        let mut rng = rand::rng();
        let answer_number = rng.random_range(0..19);
        self.ball[answer_number].to_string()
    }
}

impl Default for MagicBall {
    fn default() -> Self {
        Self::new()
    }
}

// implement unit tests for the MagicBall struct
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let magic_ball = MagicBall::new();
        assert_eq!(magic_ball.ball.len(), 20);
    }

    #[test]
    fn test_shake() {
        let magic_ball = MagicBall::new();
        let response = magic_ball.shake();
        assert!(!response.is_empty());
    }

    #[test]
    fn test_default() {
        let magic_ball = MagicBall::default();
        assert_eq!(magic_ball.ball.len(), 20);
    }

    #[test]
    fn test_multiple_shakes() {
        let magic_ball = MagicBall::new();
        let mut responses = Vec::new();
        for _ in 0..10 {
            responses.push(magic_ball.shake());
        }
        assert_eq!(responses.len(), 10);
    }
}
