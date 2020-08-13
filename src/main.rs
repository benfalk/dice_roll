use rand::prelude::*;
use clap::Clap;

/// Dice Rolling Simulator
#[derive(Clap, Debug)]
#[clap(
    version = "1.0",
    author = "Benjamin Falk <benjamin.falk@yahoo.com>"
)]
struct Opts {
    /// number of sides to the dice, should be
    /// between 1 and 255
    #[clap(short, long, default_value = "20")]
    sides: u8,

    /// number of times you want to roll the dice
    #[clap(short, long, default_value = "1")]
    times: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    let mut rng = rand::thread_rng();

    for roll in 1..(opts.times + 1) {
        let num: f32 = rng.gen();

        println!(
            "Roll #{roll:->5} : {value:>3}",
            roll = roll,
            value = (num * opts.sides as f32 + 1.) as usize
        );
    }
}
