// region:    --- Modules

use std::fs;
use std::path::{PathBuf, Path};
use derive_more::{From, Deref};
use serde::{Deserialize, Serialize};

use crate::Result; 
use crate::ais::{new_oa_client, OaClient};
use crate::ais::asst::{self, AsstId, ThreadId}; 
use crate::utils::files::{self,ensure_dir, load_from_toml, load_from_json, save_to_json};
use crate::utils::cli::ico_check;

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

        // -- Upload instructions 
        buddy.upload_instructions().await?;

        // Todo: -- Upload files

        Ok(buddy)
    }

    pub async fn upload_instructions(&self) -> Result<bool> {
        let file = self.dir.join(&self.config.instructions_file);
        if file.exists() {
            let inst_content = files::read_to_string(&file)?;
            asst::upload_instructions(&self.oac, &self.asst_id, inst_content).await?;
            println!("{} Instructions uploaded", ico_check());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    pub async fn load_or_create_conv(&self, recreate: bool) -> Result<Conv> {
        let conv_file = self.data_dir()?.join("conv.json");

        if recreate && conv_file.exists() {
            fs::remove_file(&conv_file)?;
        }

        let conv = if let Ok(conv) = load_from_json::<Conv>(&conv_file) {
            asst::get_thread(&self.oac, &conv.thread_id)
                .await
                .map_err(|_| format!("Cannot find thread_id for {:?}", conv))?;
            println!("{} Conversation loaded", ico_check());
            conv
        } else {
            let thread_id = asst::create_thred(&self.oac).await?;
            println!("{} Conversation created", ico_check());
            let conv = thread_id.into();
            save_to_json(&conv_file, &conv)?;
            conv
        };

        Ok(conv)
    }

    pub async fn chat(&self, conv: &Conv, msg: &str) -> Result<String> {
        let res = asst::run_thread_msg(
            &self.oac, 
            &self.asst_id, 
            &conv.thread_id, 
            msg
        ).await?;

        Ok(res)

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