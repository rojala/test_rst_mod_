use clap::Parser;
use fruit_salad_cli::create_fruit_salad;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Your Name <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    /// Number of fruits to include in the salad. 0 = all available fruits or enter custom fruits.
    #[clap(short, long, default_value = "0", value_name = "NUMBER", required = false)]
    number: usize,

    /// Fruits to add to the salad
    #[clap(short, long, value_name = "FRUITS", num_args = 1.., required = false)]
    fruits: Vec<String>,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // 1. Modify the program to allow the user to specify which fruits
    //    they want in their salad as command-line arguments.
    //    --> if fruits were entered use those if not then use default fruits.

    // Create the fruit salad
    let fruit_salad = create_fruit_salad(num_fruits, &opts.fruits);

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        fruit_salad
    );
}
