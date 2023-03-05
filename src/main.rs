use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    pattern: String,
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path).expect("file can not be open");

    for line in content.lines(){
        if line.contains(&args.pattern){
            println!("  -> {}", line);
        }
    }

}
