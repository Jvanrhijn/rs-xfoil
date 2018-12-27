use std::process::{Child, ChildStdin, ChildStdout, Command, Stdio};
use std::io::{self, Write, Read, BufReader, BufRead};
use std::fs::File;
use std::{thread, time};

fn start_xfoil() -> Child {
    Command::new("/usr/local/bin/xfoil")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .ok()
        .expect("Failed to execute process")
}

fn write_to_xfoil(stdin: &mut ChildStdin, command: &str) {
    stdin.write_all(command.as_bytes());
}

fn main() {
    let mut child = start_xfoil();

    let mut stdout = child.stdout.unwrap();
    let mut stdin = child.stdin.unwrap();

    let reader = BufReader::new(stdout);

    let commands = vec!["plop", "G", "\n", "naca 2414", "oper", "pacc", "outfile", "\n", "oper", "a 4"];

    thread::spawn(move || {
        for &cmd in commands.iter() {
            write_to_xfoil(&mut stdin, cmd);
            write_to_xfoil(&mut stdin, "\n");
            //thread::sleep(time::Duration::from_millis(1000));
        }
    });

    reader.lines().for_each(|line| {});

}
