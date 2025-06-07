use clap::{Parser, Subcommand};
use config::MejiroConfig;
use mejiro_cli::compile::compile;
use mejiro_cli::image::{add as image_add, list as image_list};
use mejiro_cli::new::new;
use mejiro_cli::list::list as post_list;

#[derive(Parser)]
#[command(name = "mejiro-cli", version = env!("CARGO_PKG_VERSION"), about = "Blog management CLI")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init {
        #[arg(short, long, default_value = "./mejiro.yml")]
        config_file: String,
        #[arg(short, long, default_value = "./posts")]
        posts_dir: String,
    },
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
    /// List post metadata
    List {
        #[arg(short, long, default_value = "./posts")]
        input: String,
        /// Show all posts including unpublished ones
        #[arg(short, long, default_value_t = false)]
        all: bool,
    },
    /// Manage blog images
    Image {
        #[command(subcommand)]
        command: ImageCommands,
    },
}

#[derive(Subcommand)]
enum ImageCommands {
    /// Add an image to the blog's image directory
    Add {
        path: String,
        #[arg(short, long, default_value = "./mejiro.yml")]
        config_file: String,
    },
    /// List images stored for the blog
    List {
        #[arg(short, long, default_value = "./mejiro.yml")]
        config_file: String,
    },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init {
            config_file,
            posts_dir,
        } => {
            MejiroConfig::initialize_config(&config_file, &posts_dir);
        }
        Commands::New { output } => {
            new(&output);
        }
        Commands::Compile {
            input,
            output,
            config_file,
        } => {
            compile(&input, &output, &config_file);
        }
        Commands::List { input, all } => {
            post_list(&input, all);
        }
        Commands::Image { command } => match command {
            ImageCommands::Add { path, config_file } => {
                if let Err(e) = image_add(&config_file, &path) {
                    eprintln!("❌ Failed to add image: {e}");
                }
            }
            ImageCommands::List { config_file } => {
                image_list(&config_file);
            }
        },
    }
}
