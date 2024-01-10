// region:    --- Modules

use std::path::{PathBuf, Path};
use derive_more::{From, Deref};
use serde::{Deserialize, Serialize};

use crate::{Result, 
    ais::{OaClient, asst::{AsstId, ThreadId, self}}, 
    utils::files::{ensure_dir, load_from_toml}, new_oa_client
};

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
}

/// Public functions
impl Buddy {
    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn init_from_dir(
        dir: impl AsRef<Path>,
        recreate_asst: bool,
    ) -> Result<Self> {
        let dir = dir.as_ref();

        // -- Load from the directory
        let config: Config = load_from_toml(dir.join(BUDDY_TOML))?;

        // -- Get or Create the OpenAI Assistant
        let oac = new_oa_client()?;
        let asst_id = asst::load_or_create_asst(&oac, (&config).into(), recreate_asst).await?;

        // -- Create buddy
        let buddy = Buddy {
            dir: dir.to_path_buf(),
            oac,
            asst_id,
            config
        };

        //  Todo -- Upload instructions and upload files

        Ok(buddy)
    }
}

/// Private functions 
impl Buddy {
    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.dir.join(".buddy");
        ensure_dir(&data_dir)?;  
        Ok(data_dir)
    }

    fn data_files_dir(&self) -> Result<PathBuf> {
        let dir = self.data_dir()?.join("files");
        ensure_dir(&dir)?;  
        Ok(dir)
    }
}