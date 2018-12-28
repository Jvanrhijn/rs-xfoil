use std::process::{ChildStdin, Command, Stdio};
use std::io::{Write, BufReader, BufRead};
use std::fs::File;
use std::vec::Vec;
use std::collections::HashMap;
pub mod error;
//use error::{XfoilError, Result};

enum Mode {
    Angle(f64),
    Cl(f64)
}

/// Struct tracking Xfoil configuration.
pub struct Config {
    mode: Mode,
    reynolds: Option<usize>,
    path: String,
    polar: Option<String>,
    naca: Option<String>,
    dat_file: Option<String>,
}

impl Config {

    /// Create new Xfoil configuration structure from the path to an Xfoil executable.
    pub fn new(path: &str) -> Self {
        Self{
            mode: Mode::Angle(0.0),
            reynolds: None,
            path: path.to_string(),
            polar: None,
            naca: None,
            dat_file: None
        }
    }

    /// Construct XfoilRunner from configuration
    /// panics: if no airfoil (either from polar file or NACA code) is given.
    pub fn get_runner(mut self) -> error::Result<XfoilRunner> {
        let mut command_sequence = vec!["plop", "G", "\n"]
            .into_iter().map(|x| x.to_string()).collect::<Vec<_>>();

        if let Some(naca) = self.naca {
            command_sequence.push(format!("naca {}", naca).to_string());
        } else if let Some(dat) = self.dat_file {
            command_sequence.extend_from_slice(&[
                format!("load {}", dat).to_string(),
                "".to_string()
            ]);
        } else {
            panic!("Xfoil cannot run without airfoil");
        }

        if let Some(reynolds) = self.reynolds {
            command_sequence.extend_from_slice(&[
                "oper".to_string(),
                format!("v {}", reynolds).to_string(),
                "\n".to_string()
            ]);
        }

        self.polar = if let Some(polar) = self.polar {
            command_sequence.extend_from_slice(&[
                "oper".to_string(),
                "pacc".to_string(),
                polar.to_string(),
                "\n".to_string()
            ]);
            Some(polar)
        } else {
            None
        };

        match self.mode {
            Mode::Angle(angle) => {
                command_sequence.extend_from_slice(&[
                    "oper".to_string(),
                    format!("a {}", angle).to_string(),
                    "\n".to_string()
                ])
            },
            Mode::Cl(cl) => {
                command_sequence.extend_from_slice(&[
                    "oper".to_string(),
                    format!("cl {}", cl).to_string(),
                    "\n".to_string()
                ])
            }
        }

        command_sequence.push("quit".to_string());

        Ok(XfoilRunner{
            xfoil_path: self.path,
            command_sequence,
            polar: self.polar
        })
    }

    /// Set angle of attack at which to run xfoil computation.
    /// If lift_coefficient was previously called, the state is
    /// overwritten to use an angle of attack calculation instead.
    pub fn angle_of_attack(mut self, angle: f64) -> Self {
        self.mode = Mode::Angle(angle);
        self
    }

    /// Set lift coefficient at which to run xfoil computation.
    /// If angle_of_attack was previously called, the state is
    /// overwritten to use a lift coefficient calculation instead.
    pub fn lift_coefficient(mut self, cl: f64) -> Self {
        self.mode = Mode::Cl(cl);
        self
    }

    /// Set path of polar file to save Xfoil data into.
    pub fn polar_accumulation(mut self, fname: &str) -> Self {
        self.polar = Some(fname.to_string());
        self
    }

    /// Specify a 4-digit NACA airfoil code.
    pub fn naca(mut self, code: &str) -> Self {
        self.naca = Some(code.to_string());
        self.dat_file = None;
        self
    }

    /// Specify a file containing airfoil coordinates to use in Xfoil computation.
    pub fn airfoil_polar_file(mut self, path: &str) -> Self {
        self.dat_file = Some(path.to_string());
        self.naca = None;
        self
    }

    /// Set a Reynolds number for a viscous calculation.
    pub fn reynolds(mut self, reynolds: usize) -> Self {
        self.reynolds = Some(reynolds);
        self
    }

}

pub struct XfoilRunner {
    xfoil_path: String,
    command_sequence: Vec<String>,
    polar: Option<String>
}

impl XfoilRunner {

    /// Run Xfoil calculation. This method dispatches a child process, and feeds
    /// a sequence of commands to its stdin. After the calculation finishes,
    /// it outputs the contents of the resulting polar file in a HashMap.
    /// This method panics if something goes wrong either executing the child
    /// process, or retrieving a handle to its stdin. It may return an XfoilError
    /// if anything goes wrong writing to the process or parsing its output.
    pub fn dispatch(self) -> error::Result<HashMap<String, Vec<f64>>> {
        let mut child = Command::new(&self.xfoil_path)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .ok()
            .expect("Failed to execute Xfoil");

        let mut stdin = child.stdin.as_mut()
            .expect("Failed to retrieve handle to child stdin");

        for cmd in self.command_sequence.iter() {
            Self::write_to_xfoil(&mut stdin, &cmd)?;
            Self::write_to_xfoil(&mut stdin, "\n")?;
        }

        child.wait()?;

        // TODO: parse output for errors
        //let _ = child.wait_with_output().unwrap();
        /*for c in output.stdout {
            print!("{}", c as char);
        }*/

        if let Some(polar) = &self.polar {
            self.parse_polar(polar)
        } else {
            Ok(HashMap::new())
        }

    }

    fn write_to_xfoil(stdin: &mut ChildStdin, command: &str) -> error::Result<()> {
        Ok(stdin.write_all(command.as_bytes())?)
    }

    fn parse_polar(&self, path: &str) -> error::Result<HashMap<String, Vec<f64>>> {
        let mut result = HashMap::new();
        let table_header = ["alpha", "CL", "CD", "CDp", "CM", "Top_Xtr", "Bot_Xtr"];
        for header in &table_header {
            result.insert(header.to_string(), Vec::<f64>::new());
        }
        // number of lines in Xfoil polar header
        const HEADER: usize = 13;
        for line in BufReader::new(File::open(path)?).lines().skip(HEADER-1) {
            let data = line?.split_whitespace()
                .map(|x| x.parse::<f64>().expect("Failed to parse Xfoil polar"))
                .collect::<Vec<_>>();
            for (header, value) in table_header.iter().zip(data) {
                result.get_mut::<String>(&header.to_string())
                    .expect("Failed to retrieve result HashMap")
                    .push(value);
            }
        }
        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn no_foil() {
        let _runner = Config::new("/usr/local/bin/xfoil")
            .get_runner()
            .unwrap();
    }
    
}
