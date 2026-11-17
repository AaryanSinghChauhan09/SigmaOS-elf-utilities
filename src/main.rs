//! SigmaOS-elf-utilities — thin CLI entry point.
//!
//! The library lives in `src/lib.rs`; this binary only wires argv/stdout.

use sigmaos_elf_utilities_lib::Component;

fn main() {
    let mut component = Component::new();
    if let Err(e) = component.init().and_then(|_| component.run()) {
        eprintln!("SigmaOS-elf-utilities: {e}");
        std::process::exit(1);
    }
    println!("SigmaOS-elf-utilities: ok");
}
