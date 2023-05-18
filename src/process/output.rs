use std::process::Command;
use std::io::{BufReader, BufRead};
use std::thread;

use crate::config::cfg;
use crate::process::parse;

pub fn handle_output(process: &mut Command, process_cfg: &cfg::Process) {
    let stdout = process.stdout.take().unwrap();
    let stderr = process.stderr.take().unwrap();

    // Spawn separate threads for reading.
    let stdout_t = thread::spawn(move || {
        parse_output(stdout, process_cfg);
    });

    let stderr_t = thread::spawn(move || {
        parse_output(stderr, process_cfg);
    });

    // Wait for threads.
    stdout_t.join().unwrap();
    stderr_t.join().unwrap();
}

fn parse_output(pipe: std::process::ChildStdout, process_cfg: &cfg::Process)  {
    let reader = BufReader::new(pipe);

    for line in reader.lines() {
        if let Ok(line) = line {
            // Go through each rule.
            for rule in process_cfg.rules.iter() {
                if parse::parse_line(rule.regex_match.as_str(), line.as_str()) {
                    if let Some(ref rule_actions) = rule.actions {
                        for action in rule_actions.iter() {
                            match action.action_type.as_str() {
                                "stdout" => println!("{}", line.as_str())
                            }
                        }
                    }
                }
            }
        }
    }
}