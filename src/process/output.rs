use crate::config::cfg;
use crate::process::parse;

use tokio::io::{BufReader, AsyncBufReadExt};
use tokio::process::{Child, ChildStdout, ChildStderr};

enum Output {
    StdOut(ChildStdout),
    StdErr(ChildStderr)
}

pub async fn handle_output(process: &mut Child, process_cfg: &cfg::Process) {
    let stdout_pipe = process.stdout.take().unwrap();
    let stderr_pipe = process.stderr.take().unwrap();

    // Spawn separate threads for reading.
    let stdout_fut = async {
        parse_output(process_cfg, Some(stdout_pipe), None).await;
    };

    let stderr_fut = async {
        parse_output(process_cfg, None, Some(stderr_pipe)).await;
    };

    tokio::join!(stdout_fut, stderr_fut);
}

async fn parse_output(process_cfg: &cfg::Process, out_pipe: Option<ChildStdout>, err_pipe: Option<ChildStderr>) {
    println!("Parsed output.");

    let reader: BufReader<Output>;
    if let Some(pipe) = out_pipe {
        reader = BufReader::new(Output::StdOut(pipe));
    } else if let Some(pipe) = err_pipe {
        reader = BufReader::new(Output::StdErr(pipe));
    } else {
        return;
    }
    
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await.unwrap() {
        for rule in process_cfg.rules.iter() {
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
}