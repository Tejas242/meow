use std::io::BufRead;

// Lib for parsing command-line arguments
use clap::Parser;

// Lib for printing pretty errors (literally? pretty? an error?)
use anyhow::Result;

// for Syntax highlighting
use syntect::highlighting::{Style, ThemeSet};
use syntect::parsing::SyntaxSet;
use syntect::util::as_24_bit_terminal_escaped;
use syntect::easy::HighlightFile;

/* There are a lot of custom attributes you can add to fields. For example, to say you want to use this field for the argument after -o or --output, you’d add #[arg(short = 'o', long = "output")] */

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    // pattern to look for
    pattern: String,
    // path to the file to read
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // MANUAL METHOD
    // std library in rust contains function std::env::args() which returns an iterator over the command line arguments
    // nth() method returns the nth element of the iterator (1 for 2nd element, as 0 is the program name itself)
    // expect method is a shortcut function that will make the program exit immediately when the value could not be read.
    // let pattern = std::env::args().nth(1).expect("no pattern given");
    // let path = std::env::args().nth(2).expect("no path given");

    // let args = Cli {
    //     pattern: pattern,
    //     path: std::path::PathBuf::from(path),
    // };

    // println!("pattern: {:?}, path: {:?}", pattern, path);
    // /* run as `cargo run -- some-pattern some-file`
    //    prints `pattern: "some-pattern", path: "some-file"`*/
    // Loading syntaxes and themes
    let ss = SyntaxSet::load_defaults_newlines();
    let ts = ThemeSet::load_defaults();


    // USING CLAP
    let args = Cli::parse();
    // let file = std::fs::read_to_string(&args.path)
    //     .with_context(|| format!("could not read file `{}`", &args.path.display()))?;

    let mut highlighter =
        HighlightFile::new(&args.path, &ss, &ts.themes["base16-ocean.dark"]).unwrap();

    let mut line = String::new();

    while highlighter.reader.read_line(&mut line)? > 0 {
        {
            let regions: Vec<(Style, &str)> = highlighter
                .highlight_lines
                .highlight_line(&line, &ss)
                .unwrap();
            print!("{}", as_24_bit_terminal_escaped(&regions[..], true));
        } // until NLL this scope is needed so we can clear the buffer after
        line.clear(); // read_line appends so we need to clear between lines
    }

    // ? in above line replaces this
    // let content = match file {
    //     Ok(content) => { content }, // match block needs to return something so return content
    //     Err(error) => { return Err(error.into()); }
    // };

    // Highlight each line of the file and print it with syntax highlighting
    // for line in file.lines() {
    //     // Highlight the current line
    //     let regions_result = h.highlight_line(line, &ps);

    //     // Check if there's an error in highlighting the line
    //     if let Ok(regions) = regions_result {
    //         // Convert the highlighted regions to ANSI-colored text
    //         let highlighted_line = as_24_bit_terminal_escaped(&regions, true);

    //         // Print the highlighted line
    //         println!("{}", highlighted_line);
    //     } else {
    //         // Handle the error case
    //         eprintln!("Error highlighting line: {:?}", regions_result);
    //         // Print the line as is without highlighting
    //         println!("{}", line);
    //     }
    // }

    // for whole file printing
    // println!("File content:\n{}", file);

    // for pattern finding
    // Add the `indicatif` crate as a dependency in your `Cargo.toml` file
    // ...
    // [dependencies]
    // indicatif = "0.15.0"

    // for line in file.lines() {
    //     if line.contains(&args.pattern){
    //         println!("{}", line);
    //     }
    // }

    Ok(())
}
