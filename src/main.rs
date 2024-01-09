// region:    --- Modules

use crate::ais::asst::CreateConfig;

pub use self::ais::new_oa_client;

pub use self::error::{Error, Result};

mod error;
mod ais;

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

    let asst_id = ais::asst::create(&oac, asst_config).await?;
    println!("->> asst_id: {asst_id}");
    Ok(())
}