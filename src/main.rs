mod cli;
mod commands;
mod emulator;
mod error;
mod image;
mod machine;
mod qemu;
mod ssh_cmd;
mod util;
mod view;

use clap::Parser;

fn main() {
    util::migrate();

    cli::Cli::parse()
        .command
        .ok_or(error::Error::UnknownCommand)
        .and_then(cli::dispatch)
        .map_err(error::print_error)
        .ok();
}
