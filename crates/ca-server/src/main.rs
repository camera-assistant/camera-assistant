use std::sync::LazyLock;

use clap::{CommandFactory, Parser};

/// Version of the API schema that this server speaks.
const API_SCHEMA_VERSION: u32 = 1;

/// The Camera Assistant server.
#[derive(Parser)]
#[command(version = version())]
struct Cli {}

fn main() {
    Cli::parse();
    Cli::command().print_help().expect("failed to print help");
}

/// The release version and the API schema version, as `--version` prints them.
fn version() -> &'static str {
    static VERSION: LazyLock<String> = LazyLock::new(|| {
        format!(
            "{} (API schema {API_SCHEMA_VERSION})",
            env!("CARGO_PKG_VERSION")
        )
    });
    &VERSION
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn command_line_is_valid() {
        Cli::command().debug_assert();
    }
}
