pub mod config;
pub mod cmdline;
pub mod process;

use config::cfg;
use cmdline::parse;
use process::command;

fn main() {
    // Parse command line arguments.
    let args = parse::parse_arguments();

    // Retrieve config.
    let cfg: cfg::Config = cfg::load_cfg(args.cfg_file_name);

    // Loop through each process.
    for process in cfg.processes.iter() {
        let new_process = command::spawn_process(process);

        

    }
}