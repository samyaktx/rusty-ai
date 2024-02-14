// region:    --- Modules

use std::fs;
use std::path::{PathBuf, Path};
use derive_more::{From, Deref};
use serde::{Deserialize, Serialize};

use crate::Result; 
use crate::ais::{new_oa_client, OaClient};
use crate::ais::asst::{self, AsstId, ThreadId}; 
use crate::utils::files::{self, 
    ensure_dir, load_from_toml, 
    load_from_json, save_to_json, 
    list_files, bundle_to_file
};
use crate::utils::cli::ico_check;

use self::config::Config;

mod config;

// endregion: --- Modules

const RUSTY_AI_TOML: &str = "rusty_ai.toml";

#[derive(Debug)]
pub struct RustyAI {
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
impl RustyAI {
    pub fn name(&self) -> &str {
        &self.config.name
    }

    pub async fn init_from_dir(
        dir: impl AsRef<Path>,
        recreate_asst: bool,
    ) -> Result<Self> {
        let dir = dir.as_ref();

        // -- Load from the directory
        let config: Config = load_from_toml(dir.join(RUSTY_AI_TOML))?;

        // -- Get or Create the OpenAI Assistant
        let oac = new_oa_client()?;
        let asst_id = asst::load_or_create_asst(&oac, (&config).into(), recreate_asst).await?;

        // -- Create RustyAI
        let rusty_ai = RustyAI {
            dir: dir.to_path_buf(),
            oac,
            asst_id,
            config
        };

        // -- Upload instructions 
        rusty_ai.upload_instructions().await?;

        // -- Upload files
        rusty_ai.upload_files(false).await?;

        Ok(rusty_ai)
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

    pub async fn upload_files(&self, recreate: bool) -> Result<u32> {
        let mut num_uploaded = 0;

        // the .rusty_ai/files
        let data_files_dir = self.data_files_dir()?;

        // -- Clean the .rusty_ai/file left over.
        let exclude_element = format!("*{}*", &self.asst_id);
        for file in list_files(
            &data_files_dir, Some(&["*.rs", "*.md"]), 
            Some(&[&exclude_element]),
        )? {
            let file_str = file.to_string_lossy();
            // Safegaurd
            if !file_str.contains(".rusty_ai") {
                return Err(
                    format!("Error should not delete: '{}'", file_str).into()
                );
            }
            // delete file
            fs::remove_file(&file)?;
        }

        // --- Generate and upload the .rusty_ai/files bundle files.
        for bundle in self.config.file_bundles.iter() {
            let src_dir = self.dir.join(&bundle.src_dir);

            if src_dir.is_dir() {
                let src_globs: Vec<&str> = bundle.src_globs
                    .iter()
                    .map(AsRef::as_ref)
                    .collect();

                let files = list_files(&src_dir, Some(&src_globs), None)?;

                if !files.is_empty() {
                    // Compute bundle file name
                    let buddle_file_name = format!(
                        "{}-{}-bundle-{}.{}",
                        self.name(),
                        bundle.bundle_name,
                        self.asst_id,
                        bundle.dst_ext
                    );

                    let bundle_file = self.data_files_dir()?.join(buddle_file_name);

                    // If it doesn't exist, then we will force a reupload.
                    let force_reupload = recreate || !bundle_file.exists();

                    // Rebundle no matter if exist or not (to check).
                    bundle_to_file(files, &bundle_file)?;

                    // Upload
                    let (_, uploaded) = asst::upload_file_by_name(
                        &self.oac,
                        &self.asst_id,
                        &bundle_file,
                        force_reupload,
                    ).await?;

                    if uploaded {
                        num_uploaded += 1;
                    }
                }
            }
        }

        Ok(num_uploaded)
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
impl RustyAI {
    fn data_dir(&self) -> Result<PathBuf> {
        let data_dir = self.dir.join(".rusty_ai");
        ensure_dir(&data_dir)?;  
        Ok(data_dir)
    }

    fn data_files_dir(&self) -> Result<PathBuf> {
        let dir = self.data_dir()?.join("files");
        ensure_dir(&dir)?;  
        Ok(dir)
    }
}