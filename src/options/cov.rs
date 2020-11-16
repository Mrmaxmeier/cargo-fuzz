use crate::{options::BuildOptions, project::FuzzProject, RunCommand};
use anyhow::Result;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Clone, Debug, StructOpt)]
pub struct Cov {
    #[structopt(flatten)]
    pub build: BuildOptions,

    #[structopt(required(true))]
    /// Name of the fuzz target
    pub target: String,

    #[structopt(parse(from_os_str))]
    /// Path to test cases to run for coverage, will use corpus if empty
    pub test_case: Vec<PathBuf>,
}

impl RunCommand for Cov {
    fn run_command(&mut self) -> Result<()> {
        let project = FuzzProject::find_existing()?;
        project.exec_cov(self)
    }
}
