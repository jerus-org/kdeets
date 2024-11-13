use std::fmt::Display;

use crate::{Error, LINE_CHAR, SETUP_HEADER};

use clap::Parser;
use clap_verbosity::Verbosity;
use colorful::Colorful;
use tame_index::{index::FileLock, IndexKrate, KrateName};

#[derive(Parser, Debug, Default)]
#[clap(author, version, about, long_about = None)]
pub struct Setup {
    #[clap(flatten)]
    logging: Verbosity,
    /// The name of the crate
    crate_: String,
}

impl Setup {
    pub fn run(&self) -> Result<String, Error> {
        log::info!(
            "Setting up local registry and adding crate: {}",
            self.crate_
        );
        let lock = FileLock::unlocked();
        let index = crate::get_sparce_index()?;
        let index_crate = index.krate(KrateName::crates_io(&self.crate_)?, true, &lock)?;

        let Some(index_crate) = index_crate else {
            return Err(Error::CrateNotFoundOnIndex);
        };

        let output = SetupTestOutput::new(index_crate);

        Ok(output.to_string())
    }
}

struct SetupTestOutput {
    #[allow(dead_code)]
    index_crate: IndexKrate,
    header: String,
}

impl SetupTestOutput {
    fn new(index_crate: IndexKrate) -> Self {
        let mut header = String::from("\n  ");
        header.push_str(SETUP_HEADER);
        header.push(' ');
        header.push_str(index_crate.name().cyan().to_string().as_str());
        header.push('.');
        header.push_str("\n  ");
        let mut i = 0;
        while i < SETUP_HEADER.len() + 2 + index_crate.name().len() {
            header.push(LINE_CHAR);
            i += 1;
        }
        header.push('\n');

        Self {
            index_crate,
            header,
        }
    }
}

impl Display for SetupTestOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.header)?;
        Ok(())
    }
}
