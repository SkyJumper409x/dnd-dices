//use rand::Rng;
use rand::distributions::{Distribution, Uniform};
use text_io::read;

fn main() {
    println!("What dice you want to roll? d3; d6; d10; d12; d20");
    let input: String = read!();
    let out: String = roll(&input);
    println!("{}", out);
}

fn roll(input: &str) -> String {
    let mut rng = rand::thread_rng();

    let d3 = Uniform::from(1..3);
    let d6 = Uniform::from(1..6);
    let d10 = Uniform::from(1..10);
    let d12 = Uniform::from(1..12);
    let d20 = Uniform::from(1..20);
    match input {
        "d3" => d3.sample(&mut rng).to_string(),
        "d6" => d6.sample(&mut rng).to_string(),
        "d10" => d10.sample(&mut rng).to_string(),
        "d12" => d12.sample(&mut rng).to_string(),
        "d20" => d20.sample(&mut rng).to_string(),
        _ => "no / wrong input".to_string(),
    }
}
