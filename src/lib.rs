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
    command_valid: bool,
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
            command_valid: false,
            xfoil_path: path.to_string(),
            command_sequence,
            output: Vec::<String>::new()
        }
    }

    pub fn dispatch(mut self) -> Result<HashMap<String, Vec<f64>>> {
        let mut child = Command::new(&self.xfoil_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()
            .expect("Failed to execute Xfoil");

        let mut stdin = child.stdin.as_mut().unwrap();

        for cmd in self.command_sequence.iter() {
            Self::write_to_xfoil(&mut stdin, &cmd);
            Self::write_to_xfoil(&mut stdin, "\n");
        }

        let output = child.wait_with_output().unwrap();
        for c in output.stdout {
            print!("{}", c as char);
        }

        Ok(HashMap::new())

    }

    pub fn polar_accumulation(mut self, fname: &str) -> Self {
        self.command_sequence.extend_from_slice(&[
            "oper".to_string(),
            "pacc".to_string(),
            fname.to_string(),
            "\n".to_string()
        ]);
        self
    }

    pub fn naca(mut self, code: &str) -> Self {
        self.command_sequence.insert(0, format!("naca {}\n", code).to_string());
        self.command_valid = true;
        self
    }

    pub fn airfoil_polar_file(mut self, path: &str) -> Self {
        self.command_sequence.extend_from_slice(&[
            format!("load {}", path).to_string(),
            "".to_string()
        ]);
        self
    }

    pub fn reynolds(mut self, reynolds: usize) -> Self {
        self.command_sequence.extend_from_slice(&[
            "oper".to_string(),
            format!("v {}", reynolds).to_string(),
            "\n".to_string()
        ]);
        self
    }

    pub fn angle_of_attack(mut self, angle: f64) -> Self {
        self.command_sequence.extend_from_slice(&[
            "oper".to_string(),
            format!("a {}", angle).to_string(),
            "\n".to_string()
        ]);
        self
    }

    fn write_to_xfoil(stdin: &mut ChildStdin, command: &str) {
        stdin.write_all(command.as_bytes());
    }

    fn parse_output(&self) {

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
