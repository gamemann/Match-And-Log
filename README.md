# Match & Log
**Work In Progress & Not Finished!**
My first Rust program I'm using to learn more about Rust as a programming language.

This program spawns processes specified in a JSON config file. It then parses each line from the process's `stdout` and `stderr` pipes and if the line matches a Regex rule, action(s) may be performed such as logging to the Rust program's `stdout` or sending a HTTP request.

## Credits
* [Christian Deacon](https://github.com/gamemann)