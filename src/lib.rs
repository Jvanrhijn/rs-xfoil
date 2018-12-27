use std::process::{ChildStdin, Command, Stdio};
use std::{error, fmt, thread};
use std::io::{self, Write, Read, BufReader, BufRead};
use std::vec::Vec;
use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
pub struct XfoilError;

impl fmt::Display for XfoilError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Internal xfoil error")
    }
}

impl error::Error for XfoilError {
    fn description(&self) -> &str {
        "Error occured in xfoil calculation"
    }

    fn cause(&self) -> Option<&error::Error> {
        None
    }
}

type Result<T> = std::result::Result<T, XfoilError>;

pub struct XfoilRunner {
    xfoil_path: String,
    command_sequence: Vec<String>,
    output: Vec<String>,
}

impl XfoilRunner {
    pub fn new(path: &str) -> Self {
        let command_sequence = vec![
            "plop".to_string(),
            "G".to_string(),
            "\n".to_string(),
        ];
        Self{
            xfoil_path: path.to_string(),
            command_sequence,
            output: Vec::<String>::new()
        }
    }

    pub fn dispatch(self) -> Result<HashMap<String, Vec<f64>>> {
        let mut child = Command::new(&self.xfoil_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()
            .expect("Failed to execute Xfoil");
        let mut stdin = child.stdin.unwrap();
        let mut stdout = child.stdout.unwrap();

        //let reader = BufReader::new(stdout);

        thread::spawn(move || {
            for cmd in self.command_sequence.iter() {
                Self::write_to_xfoil(&mut stdin, &cmd);
                Self::write_to_xfoil(&mut stdin, "\n");
            }
        });

        //reader.lines().for_each(|line| self.output.push(line.unwrap()));

        Ok(HashMap::new())

    }

    fn write_to_xfoil(stdin: &mut ChildStdin, command: &str) {
        stdin.write_all(command.as_bytes());
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn create_runner() {
        let mut runner = XfoilRunner::new("/usr/local/bin/xfoil");
        runner.dispatch().unwrap();
    }
}
