#![forbid(unsafe_code)]
use std::env::args;
use std::io::{self, BufRead, Write};
use std::process;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

#[derive(Debug, PartialEq)]
struct LineExecError {
    message: String,
}

impl LineExecError {
    fn new(message: &str) -> LineExecError {
        LineExecError {
            message: message.to_string(),
        }
    }
}

impl From<io::Error> for LineExecError {
    fn from(error: io::Error) -> Self {
        LineExecError {
            message: error.to_string(),
        }
    }
}

type ResultOrError<T> = Result<T, LineExecError>;

struct LineExec {
    command: String,
    variable: String,
    stdin: io::Stdin,
    stdout: io::Stdout,
    stderr: io::Stderr,
}

impl LineExec {
    fn new() -> ResultOrError<Self> {
        let args = LineExec::fetch_cmdline_args()?;
        let variable = String::from(&args[1]);
        let command = String::from(&args[2]);
        Ok(LineExec {
            command,
            variable,
            stdin: io::stdin(),
            stdout: io::stdout(),
            stderr: io::stderr(),
        })
    }

    fn fetch_cmdline_args() -> ResultOrError<Vec<String>> {
        let args: Vec<String> = args().collect();
        if args.len() == 2 && args[1] == "--version" {
            println!("{} v{}", &args[0], VERSION);
            process::exit(0);
        }
        if args.len() != 3 {
            eprintln!("Usage: {} <variable-name> <command-to-execute>", &args[0]);
            eprintln!("Example: ls | {} v 'echo $v'", &args[0]);
            return Err(LineExecError::new("Wrong number of command-line arguments"));
        }
        Ok(args)
    }

    fn run_command(&self, buf: String) -> ResultOrError<process::Output> {
        let command = self
            .command
            .replace(&format!("${}", self.variable), buf.trim());
        Ok(process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?)
    }

    fn exec(&self) -> ResultOrError<()> {
        let mut stdin = self.stdin.lock();
        let mut stdout = self.stdout.lock();
        let mut stderr = self.stderr.lock();
        let mut status_vec: Vec<process::ExitStatus> = Vec::new();
        loop {
            let mut buf = String::new();
            let bytest_read = stdin.read_line(&mut buf)?;
            if bytest_read == 0 {
                break;
            }
            let command_output = self.run_command(buf)?;
            stdout.write_all(&command_output.stdout)?;
            if !command_output.status.success() {
                stderr.write_all(&command_output.stderr)?;
            }
            status_vec.push(command_output.status);
        }
        if status_vec.into_iter().all(|s| s.success()) {
            Ok(())
        } else {
            Err(LineExecError::new(
                "Error running at least on the line commands",
            ))
        }
    }
}

fn main() -> ResultOrError<()> {
    let lnx = LineExec::new()?;
    lnx.exec()?;
    Ok(())
}
