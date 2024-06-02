use lfsr::Lfsr;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
/// A calculator for strength training.
struct Cli {
    /// The data size in bits
    #[arg(short, long)]
    data_size: usize,

    /// The state size in bits
    #[arg(short, long)]
    state_size: usize,

    /// The polynomial
    // FIXME: Parse as hex
    #[arg(short, long)]
    polynomial: u64,

    /// Include initial state
    #[arg(short, long)]
    initial: bool,
}

fn main() {
    let cli = Cli::parse();
    let lfsr = Lfsr::new(cli.data_size, cli.state_size, cli.polynomial, cli.initial);
    println!("{}", lfsr);
}
