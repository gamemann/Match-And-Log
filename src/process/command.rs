use std::process::{Command, Stdio};

use crate::config::cfg;

pub fn spawn_process(process: &cfg::Process) -> &mut Command {
    let mut p = Command::new(process.command.as_str())
    .stdout(Stdio::piped())
    .stderr(Stdio::piped());

    if let Some(ref process_env) = process.env {
        for env in process_env.iter() {
            p.env(env.name.as_str(), env.value.as_str());
        }
    }

    // Spawn process.
    p.spawn().expect("Error spawning process.");

    return p;
}