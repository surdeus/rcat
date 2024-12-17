use std::env;
use std::fs::File;
use std::io;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    // Skipping the first argument.
    let args: Vec<String> = args[1..].to_vec();
    if args.len() == 0 {
        // Handle the standard input to output case.
        io::copy(&mut io::stdin(), &mut io::stdout())?;
        return Ok(());
    }

    for arg in args {
        let mut file = match File::open(arg) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("error: {:?}", err);
                continue;
            }
        };
        io::copy(&mut file, &mut io::stdout())?;
    }

    Ok(())
}
