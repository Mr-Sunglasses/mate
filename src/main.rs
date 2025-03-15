use std::process::Command;

// todo: Add tests

fn main() {
    let ansi_shadow_ascii = format!("
███╗   ███╗ █████╗ ████████╗███████╗
████╗ ████║██╔══██╗╚══██╔══╝██╔════╝
██╔████╔██║███████║   ██║   █████╗  
██║╚██╔╝██║██╔══██║   ██║   ██╔══╝  
██║ ╚═╝ ██║██║  ██║   ██║   ███████╗
╚═╝     ╚═╝╚═╝  ╚═╝   ╚═╝   ╚══════╝
");
    let version = "0.1.0";

    let help = format!(
        "
{}

mate v{}

Add co-authors to your git commits.

USAGE:
    mate --name <name> --email <email>
    mate -n <name> -e <email>

OPTIONS:
    -h, --help      Prints help information
    -v, --version   Prints version information
    -n, --name      Name of the co-author
    -e, --email     Email of the co-author

SUBCOMMANDS:
    help            Prints this message
    version         Prints version information
",
        ansi_shadow_ascii,
        version
    );

    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        println!("{}", help);
        eprintln!("No arguments provided. Use 'mate --help' for more information.");
        return;
    }
    match args[1].as_str() {
        "-h" | "--help" | "help" => {
            println!("{}", help);
            return;
        }
        "-v" | "--version" | "version" => {
            println!("mate v{}", version);
            return;
        }
        // todo: Make the author name or email optional and must have atleast one of them.
        "-n" | "--name" => {
            if args.len() < 3 {
                eprintln!("Please provide the name of the co-author. Use 'mate --help' for more information.");
                return;
            }
            if args.len() <= 4 {
                eprintln!("Please provide the email of the co-author. Use 'mate --help' for more information.");
                return;
            }
            match args[3].as_str() {
                "-e" | "--email" => {
                    if args.len() > 5 {
                        eprintln!("Invalid command. Use 'mate --help' for more information.");
                        return;
                    }
                    add_co_author(args[2].to_string(), args[4].to_string());
                    println!(
                        "Co-author name: {} and email: {} added successfully.",
                        args[2].to_string(),
                        args[4].to_string()
                    );
                }
                _ => {
                    eprintln!("Invalid command. Use 'mate --help' for more information.");
                    return;
                }
            }
        }
        _ => {
            eprintln!("Invalid command. Use 'mate --help' for more information.");
            return;
        }
    }
}

fn add_co_author(name: String, email: String) {
    // todo: Make the author name or email optional and must have atleast one of them.

    let mut _author;
    if name.is_empty() || email.is_empty() {
        eprintln!("Name and email both cannot be empty.");
        return;
    } else if email.is_empty() {
        _author = format!("Co-authored-by: {}\n", name.to_string());
    } else if name.is_empty() {
        _author = format!("Co-authored-by: <{}>\n", email.to_string());
    } else {
        _author = format!(
            "Co-authored-by: {} <{}>\n",
            name.to_string(),
            email.to_string()
        );
    }

    let mut command = Command::new("git");
    command
        .arg("commit")
        .arg("--amend")
        .arg(format!("--trailer={}", _author));

    let output = command.status().expect("Failed to execute command.");

    if output.success() {
        println!("Co-author added successfully.");
    } else {
        eprintln!("Failed to add co-author.");
    }
}
