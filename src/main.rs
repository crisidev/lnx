#![forbid(unsafe_code)]
//! Utility to map lines in stdin onto command arguments to be executed.

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// LineExec error structure.
#[derive(Debug)]
struct Error {
    #[allow(dead_code)]
    message: String,
}

impl Error {
    /// Generate a new error from a message string.
    fn new(message: &str) -> Error {
        Error {
            message: message.to_string(),
        }
    }
}

/// Implement `io::Error` for `Error`.
impl From<std::io::Error> for Error {
    fn from(error: std::io::Error) -> Self {
        Error {
            message: error.to_string(),
        }
    }
}

/// Handy result type.
type Result<T> = std::result::Result<T, Error>;

/// Structure holding `command` and `variable` from the command line arguments
/// as well as `stdin`, `stdout` and `stderr`.
struct LineExec {
    command: String,
    variable: String,
    stdin: std::io::Stdin,
    stdout: std::io::Stdout,
    stderr: std::io::Stderr,
}

impl LineExec {
    /// Create a new LineExec structure.
    fn new() -> Self {
        let args = LineExec::cmdline_args();
        let variable = String::from(&args[1]);
        let command = String::from(&args[2]);
        LineExec {
            command,
            variable,
            stdin: std::io::stdin(),
            stdout: std::io::stdout(),
            stderr: std::io::stderr(),
        }
    }

    /// Validate the command line arguments.
    fn cmdline_args() -> Vec<String> {
        let args: Vec<String> = std::env::args().collect();
        if args.len() == 2 && args[1] == "--version" {
            println!("{} v{}", &args[0], VERSION);
            std::process::exit(0);
        }
        if args.len() != 3 {
            eprintln!("Error: Wrong number of command-line arguments\n");
            eprintln!("Usage: {} <variable-name> <command-to-execute>", &args[0]);
            eprintln!("Example: ls | {} v 'echo $v'", &args[0]);
            std::process::exit(1);
        }
        args
    }

    /// Shellout a command using POSIX `sh` as base shell.
    fn command(&self, buf: String) -> Result<std::process::Output> {
        let command = self
            .command
            .replace(&format!("${}", self.variable), buf.trim());
        Ok(std::process::Command::new("sh")
            .arg("-c")
            .arg(command)
            .output()?)
    }

    /// Execute the command pipeline.
    fn exec(&self) -> Result<()> {
        let mut stdin = self.stdin.lock();
        let mut stdout = self.stdout.lock();
        let mut stderr = self.stderr.lock();
        let mut status_vec: Vec<std::process::ExitStatus> = Vec::new();
        loop {
            let mut buf = String::new();
            let bytest_read = std::io::BufRead::read_line(&mut stdin, &mut buf)?;
            if bytest_read == 0 {
                break;
            }
            let command_output = self.command(buf)?;
            std::io::Write::write_all(&mut stdout, &command_output.stdout)?;
            if !command_output.status.success() {
                std::io::Write::write_all(&mut stderr, &command_output.stderr)?;
            }
            status_vec.push(command_output.status);
        }
        if status_vec.into_iter().all(|s| s.success()) {
            Ok(())
        } else {
            Err(Error::new("Error running at least one the line commands"))
        }
    }
}

/// Entrypoint.
fn main() -> ! {
    LineExec::new()
        .exec()
        .map_err(|e| {
            eprintln!("Error: {}", e.message);
            std::process::exit(1);
        })
        .unwrap();
    std::process::exit(0);
}
