// region:    --- Modules

use std::path::PathBuf;
use derive_more::{From, Deref};
use serde::{Deserialize, Serialize};

use crate::{Result, ais::{OaClient, asst::{AsstId, ThreadId}}};

use self::config::Config;

mod config;

// endregion: --- Modules

const BUDDY_TOML: &str = "buddy.toml";

#[derive(Debug)]
pub struct Buddy {
    dir: PathBuf,
    oac: OaClient,
    asst_id: AsstId,
    config: Config,
}

#[derive(Debug, From, Deref, Deserialize, Serialize)]
pub struct Conv {
    thread_id: ThreadId,
    other: String,
}

/// Public functions
impl Buddy {
    
}

/// Private functions 
impl Buddy {
    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.dir.join(".buddy");
        // ensure_dir(&data_dir)?;  // FIXME
        Ok(data_dir)
    }

    fn data_files_dir(&self) -> Result<PathBuf> {
        let dir = self.data_dir()?.join("files");
        // ensure_dir(&dir)?;  // FIXME
        Ok(dir)
    }
}