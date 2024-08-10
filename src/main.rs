static HELP: &str = "Usage: sstrip [OPTIONS] FILE...
Remove all nonessential bytes from executable ELF files.

  -z, --zeroes        Also discard trailing zero bytes.
      --help          Display this help and exit.
      --version       Display version information and exit.";

fn main() {
    print_version();
}

fn print_version() {
    println!("sstrip Version {}", env!("CARGO_PKG_VERSION"));
}
