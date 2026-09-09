fn main() -> anyhow::Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.iter().any(|arg| matches!(arg.as_str(), "-h" | "--help")) {
        println!(
            "Diple — a focused terminal code-review workbench\n\n\
             Usage: diple [OPTIONS] [REPO]\n\n\
             Options:\n  \
               --poll <MILLISECONDS>  Refresh interval (minimum 200, default 2000)\n  \
               --base <REF>           Base revision for branch review\n  \
               --theme <NAME>         Color theme\n  \
               --wrap <on|off>        Initial line-wrapping mode\n  \
               -h, --help             Print help\n  \
               -V, --version          Print version"
        );
        return Ok(());
    }
    if args.iter().any(|arg| matches!(arg.as_str(), "-V" | "--version")) {
        println!("diple {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }
    diple::run()
}
