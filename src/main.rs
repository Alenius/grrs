use structopt::StructOpt;

// search for pattern in a file and display the lines that contain it
#[derive(StructOpt)]
struct Cli {
    // the pattern to look for
    pattern: String,
    // the path to the file to read
    #[structopt(parse(from_os_str))]
    path: std::path::PathBuf,
}

fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}

fn main() {
    let args = Cli::from_args();

    let content = std::fs::read_to_string(&args.path).expect("could not read file");

    find_matches(&content, &args.pattern, &mut std::io::stdout())
}

#[test]
fn find_a_match() {
    let mut result = Vec::new();
    find_matches("lorem ipsum\nsit amet", "lorem", &mut result);
    assert_eq!(result, b"lorem ipsum\n")
}
