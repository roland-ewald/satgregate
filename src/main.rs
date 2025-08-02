use clap::Parser;
use std::{fs, io::Error, io::ErrorKind, path::PathBuf, result::Result};


#[derive(Parser)]
struct Cli {
    #[arg(
        long = "family-structure-csv",
        help = "The CSV file describing the family structure."
    )]
    family_structure_csv: PathBuf,
}
impl Cli {
    fn validate(self: &Cli) -> Result<(), Error> {
        if !self.family_structure_csv.is_file() {
            Err(Error::new(
                ErrorKind::InvalidInput,
                format!("The family structure CSV file '{:?}' is not a file.", self.family_structure_csv),
            ))
        } else {
            Ok(())
        }
    }
}

fn main() {
    let args = Cli::parse();
    let validation_result = args.validate();
    if validation_result.is_err() {
        eprintln!(
            "Stopping, as input parameters are invalid: '{:?}'.",
            validation_result.err()
        );
    } else {
        // - Read CSV with relationships and affected/unaffected status
        // - Convert this to a SAT problem (if problem is small enough and file large enough, consider precomputing all combinations into a lookup table)
        // - Add FILTER terms to the input VCFs, denoting all plausible inheritance patterns
        println!("Done.");
    }
}