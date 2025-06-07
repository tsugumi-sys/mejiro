use clap::{Parser, Subcommand};
use mejiro_cli::new;

#[derive(Parser)]
#[command(name = "mejiro-cli", version = env!("CARGO_PKG_VERSION"), about = "Blog management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new blog post
    New {
        #[arg(short, long, default_value = "./posts")]
        output: String,
    },
    /// Build HTML from published blog posts
    Compile {
        #[arg(short, long, default_value = "./posts")]
        input: String,
        #[arg(short, long, default_value = "./public")]
        output: String,
        #[arg(short, long, default_value = "./mejiro.yml")]
        config_file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::New { output } => {
            new::create_new_post(&output);
        }
        Commands::Compile {
            input,
            output,
            config_file,
        } => {
            println!("Building site from input: {}", input);
            println!("Output will be in: {}", output);
            println!("Using config file: {}", config_file);
        }
    }
}
