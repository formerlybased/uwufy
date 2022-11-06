use clap::Parser;

#[derive(Parser)]
#[command(author, version, about = "CLI tool to uwufy/owofy your text.", long_about = None)]
struct Input {
    #[arg(short, long)]
    text:Option<String>,
    #[arg(short, long)]
    file: Option<std::path::PathBuf>,
    #[arg(short, long)]
    owo: bool,
    #[arg(short, long)]
    mixed: bool,
}

fn main() {
    let args = Input::parse();

    if args.text != None {
        let mut text = String::from("");

        if args.mixed {
            text = replace(&args.text.unwrap(), 'u', "uwu");
            text = replace(&text, 'o', "owo");
        }

        else {
            if !args.owo {
                text = replace(&args.text.as_ref().unwrap(), 'u', "uwu");
            }

            if args.owo {
                text = replace(&args.text.unwrap(), 'o', "owo");
            }
        }

        println!("{}", text);
    }

    if args.file != None {
        let content = std::fs::read_to_string(&args.file.unwrap())
            .expect("File not found!");

        for l in content.lines() {
            if args.mixed {
                let text = replace(&String::from(l), 'u', "uwu");
                println!("{}", &replace(&text, 'o', "owo"));
            }

            else {
                if !args.owo {
                    println!("{}", &replace(&String::from(l), 'u', "uwu"));
                }
                
                if args.owo {
                    println!("{}", &replace(&String::from(l), 'o', "owo"));
                }
            }
        }
    }
}

fn replace(source: &String, to_replace: char, replacement: &str) -> String {
        let mut text = String::from("");
        for c in source.chars() {
            if c == to_replace {
                text.push_str(replacement);
            }
            else {
                text.push(c);
            }
        }
        return text.to_string();
}
