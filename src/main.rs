use std::{env, error::Error, fs, process};

use dppi::evaluator::Evaluator;

fn main() -> Result<(), Box<dyn Error>> {
    let files = env::args()
        .into_iter()
        .filter(|file| file.ends_with(".dppi"))
        .collect::<Vec<String>>();

    for (i, file) in files.iter().enumerate() {
        let source = fs::read_to_string(file).expect("Failed to read a file");

        let mut evaluator = Evaluator::new(&source);

        println!("Output for file {}: {}:", i + 1, file);

        evaluator.eval_program().unwrap_or_else(|err| {
            eprintln!("| DPPI Error |\n{err}");
            process::exit(1);
        });

        println!("------------------------------------");
    }

    Ok(())
}
