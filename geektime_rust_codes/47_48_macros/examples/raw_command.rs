#[allow(dead_code)]
#[derive(Debug)]
pub struct Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: Option<String>,
}

#[derive(Debug, Default)]
pub struct CommandBuilder {
    executable: Option<String>,
    args: Option<Vec<String>>,
    env: Option<Vec<String>>,
    current_dir: Option<String>,
}

impl Command {
    pub fn builder() -> CommandBuilder {
        Default::default()
    }
}

impl CommandBuilder {
    pub fn executable(mut self, v: String) -> Self {
        self.executable = Some(v.to_owned());
        self
    }

    pub fn args(mut self, v: Vec<String>) -> Self {
        self.args = Some(v.to_owned());
        self
    }

    pub fn env(mut self, v: Vec<String>) -> Self {
        self.env = Some(v.to_owned());
        self
    }

    pub fn current_dir(mut self, v: String) -> Self {
        self.current_dir = Some(v.to_owned());
        self
    }

    pub fn build(mut self) -> Result<Command, &'static str> {
        Ok(Command {
            executable: self.executable.take().ok_or("executable must be set")?,
            args: self.args.take().ok_or("args must be set")?,
            env: self.env.take().ok_or("env must be set")?,
            current_dir: self.current_dir.take(),
        })
    }
}

fn main() {
    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .build()
        .unwrap();
    assert!(command.current_dir.is_none());

    let command = Command::builder()
        .executable("cargo".to_owned())
        .args(vec!["build".to_owned(), "--release".to_owned()])
        .env(vec![])
        .current_dir("..".to_owned())
        .build()
        .unwrap();
    assert!(command.current_dir.is_some());
    println!("{:?}", command);
}