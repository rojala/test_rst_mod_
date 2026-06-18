//command-line tool that reads a CSV file and prints the contents of the file as a DataFrame
use clap::Parser;
const CSV_FILE: &str = "src/data/global-life-expt-2022.csv";

#[derive(Parser)]
//add extended help
#[clap(
    version = "1.0",
    author = "Noah Gift",
    about = "A command-line tool that reads a CSV file and prints the contents of the file as a DataFrame",
    after_help = "Example: cargo run -- print --rows 3"
)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Commands>,
}

#[derive(Parser)]
enum Commands {
    Print {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
    Describe {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Schema {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Shape {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
    },
    Sort {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long = "sort-by", alias = "year", default_value = "2020")]
        sort_by: String,
        #[clap(long, default_value = "10")]
        rows: usize,
        #[clap(long, default_value = "true")]
        order: bool,
    },
    Filter {
        #[clap(long, default_value = CSV_FILE)]
        path: String,
        #[clap(long = "by-col", default_value = "2020")]
        by_col: String,
        #[clap(long, default_value = "gt")]
        op: String,
        #[clap(long)]
        value: String,
        #[clap(long, default_value = "10")]
        rows: usize,
    },
}

fn main() {
    let args = Cli::parse();
    match args.command {
        Some(Commands::Print { path, rows }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.head(Some(rows)));
        }
        Some(Commands::Describe { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df);
        }
        Some(Commands::Schema { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.schema());
        }
        Some(Commands::Shape { path }) => {
            let df = polarsdf::read_csv(&path);
            println!("{:?}", df.shape());
        }
        Some(Commands::Sort {
            path,
            sort_by,
            rows,
            order,
        }) => {
            let df = polarsdf::read_csv(&path);
            //sort the dataframe by the requested column and order
            let df2 = match polarsdf::sort_by_column(&df, &sort_by, order) {
                Ok(df2) => df2,
                Err(error) => {
                    eprintln!("{error}");
                    return;
                }
            };

            //print the first "rows" of the dataframe
            println!("{:?}", df2.head(Some(rows)));
        }
        Some(Commands::Filter {
            path,
            by_col,
            op,
            value,
            rows,
        }) => {
            let df = polarsdf::read_csv(&path);
            let filtered_df = match polarsdf::filter_by_condition(&df, &by_col, &op, &value) {
                Ok(filtered_df) => filtered_df,
                Err(error) => {
                    eprintln!("{error}");
                    return;
                }
            };

            println!("{:?}", filtered_df.head(Some(rows)));
        }
        None => {
            println!("No subcommand was used");
        }
    }
}
