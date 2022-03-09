use std::process::Command;

pub mod users {
    pub const TESTER: &str = "tester";
}

#[derive(Debug)]
pub struct Docker {
    id: String,
}

impl Docker {
    pub fn run(image: &str) -> Self {
        let result = Command::new("docker")
            .arg("run")
            .arg("-d")
            .arg("-i")
            .arg("--rm")
            .arg(image)
            .arg("bash")
            .output()
            .expect("'docker run' failed to start");

        let id = String::from_utf8(result.stdout)
            .expect("cannot read 'docker run' output")
            .replace('\n', "");
        Self { id }
    }

    pub fn stop(&self) {
        Command::new("docker")
            .arg("stop")
            .arg(&self.id)
            .output()
            .expect("'docker stop' failed to start");
    }

    pub fn exec(&self, command: &str, args: ExecArgs) -> ExecResult {
        let result = Command::new("docker")
            .arg("exec")
            .args(&args.to_args())
            .arg(&self.id)
            .arg("sh")
            .arg("-c")
            .arg(command)
            .output()
            .expect("'docker exec' failed to start");

        if result.status.success() {
            ExecResult::Success(
                String::from_utf8(result.stdout).unwrap_or_else(|_| String::from("NO_STDOUT")),
            )
        } else {
            ExecResult::Error(
                String::from_utf8(result.stderr).unwrap_or_else(|_| String::from("NO_STDERR")),
            )
        }
    }
}

#[derive(Debug)]
pub enum ExecResult {
    Success(String),
    Error(String),
}

pub enum ExecArgs {
    Default,
    User(String),
}

impl ExecArgs {
    pub fn to_args(&self) -> Vec<&str> {
        match self {
            ExecArgs::Default => Vec::new(),
            ExecArgs::User(user) => vec!["--user", user],
        }
    }
}
