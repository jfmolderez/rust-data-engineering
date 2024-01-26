use clap::Parser;
use cli_salad::create_fruit_salad;

#[derive(Parser)]
#[clap(version = "1.0", author = "Your Name", about = "Creates a fruit salad")]

struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Create a fruit salad
    let salad = create_fruit_salad(opts.number);

    // Print the fruit salad in human readable format with a count of fruits
    println!(
        "Created fruit salad with {} fruits : {:?}",
        num_fruits, salad
    );
}
