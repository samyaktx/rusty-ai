// region:    --- Modules

use crate::ais::asst::CreateConfig;

pub use self::ais::new_oa_client;

pub use self::error::{Error, Result};

mod error;
mod ais;

// endregion: --- Modules

// region:    --- Modules
// endregion: --- Modules

#[tokio::main]
async fn main() {
   println!();

   match start().await {
       Ok(_) => println!("\nBye!\n"),
       Err(e) => println!("\nError: {}\n", e),
   }
}

async fn start() -> Result<()> {
    let oac = new_oa_client()?;
    let asst_config = CreateConfig {
        name: "rusty-ai-buddy".to_string(),
        model: "gpt-3.5-turbo-1106".to_string()
    };

    let asst_id = ais::asst::load_or_create_asst(&oac, asst_config, false).await?;
    println!("->> asst_id: {asst_id}");
    Ok(())
}