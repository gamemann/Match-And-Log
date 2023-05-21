use tokio::process::{Command, Child};
use std::process::Stdio;

use crate::config::cfg;
use crate::process::{parse, output};

use tokio::io::{BufReader, AsyncBufReadExt};

pub async fn spawn_process(process_cfg: &cfg::Process) {
    let mut p: Command = Command::new(process_cfg.command.as_str());
    p.stdout(Stdio::piped());
    p.stderr(Stdio::piped());

    if let Some(ref process_env) = process_cfg.env {
        for env in process_env.iter() {
            p.env(env.name.as_str(), env.value.as_str());
        }
    }

    // Spawn process.
    let mut spawned: Child = p.spawn().expect("Error spawning process.");
    /*
    let stdout = spawned.stdout.take().unwrap();
    let stderr = spawned.stderr.take().unwrap();
    let reader = BufReader::new(stdout);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.expect("spawn_process() :: Error with retrieving lines") {
        for rule in process.rules.iter() {
            if parse::parse_line(rule.regex_match.as_str(), line.as_str()) {
                if let Some(ref rule_actions) = rule.actions {
                    for action in rule_actions.iter() {
                        match action.action_type.as_str() {
                            "stdout" => println!("{}", line),
                            _ => {}
                        }
                    }
                }
            }
        }
    }
    */

    output::handle_output(&mut spawned, process_cfg).await;
}