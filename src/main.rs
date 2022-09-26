use std::process::exit;

mod errors;
mod cli;
mod config;
mod container;

use errors::exit_with_retcode;

fn main() {
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_with_retcode(container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_retcode());
        }
    };
}
