// region:    --- Modules

use textwrap::wrap;

use crate::buddy::Buddy;
use crate::utils::cli::{prompt, txt_res, ico_res};
pub use self::ais::new_oa_client;
pub use self::error::{Error, Result};

mod error;
mod ais;
mod buddy;
mod utils;

// endregion: --- Modules

#[tokio::main]
async fn main() {
   println!();

   match start().await {
       Ok(_) => println!("\n{} Bye, See you\n", ico_res()),
       Err(e) => println!("\nError: {}\n", e),
   }
}

// region:    --- Types

#[derive(Debug)]
enum Cmd {
    Quit,
    Chat(String),
    RefreshAll,
    RefreshConv,
    RefreshInst,
    RefreshFiles,
    Help,
}

impl  Cmd {
    fn from_input(intput: impl Into<String>) -> Self {
        let input = intput.into();

        if input == ":q" {
            Self::Quit
        } else if input == ":ra" || input == ":RA" {
            Self::RefreshAll
        } else if input == ":ri" || input == ":RI" {
            Self::RefreshInst
        } else if input == ":rf" || input == ":RF" {
            Self::RefreshFiles
        } else if input == ":rc" || input == ":RC" {
            Self::RefreshConv
        } else if input == ":h" || input == ":H" {
            Self::Help
        } else {
            Self::Chat(input)
        }
    }
}

// endregion: --- Types

const DEFAULT_DIR: &str = "buddy";

async fn start() -> Result<()> {
    let mut buddy = Buddy::init_from_dir(DEFAULT_DIR, false).await?;

    let mut conv = buddy.load_or_create_conv(false).await?;
    
    loop {
        println!();
        let input = prompt("rusty-ai query")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,
            Cmd::Chat(msg) => {
                let res = buddy.chat(&conv, &msg).await?;
                let res = wrap(&res, 80).join("\n");
                println!("{} {}",  ico_res(), txt_res(res));
            },
            Cmd::RefreshAll => {
                buddy = Buddy::init_from_dir(DEFAULT_DIR, true).await?;
                conv = buddy.load_or_create_conv(true).await?;
            },
            Cmd::RefreshConv => {
                conv = buddy.load_or_create_conv(true).await?;
            },
            Cmd::RefreshInst => {
                buddy.upload_instructions().await?;
                conv = buddy.load_or_create_conv(true).await?;
            },
            Cmd::RefreshFiles => {
                buddy.upload_files(true).await?;
                conv = buddy.load_or_create_conv(true).await?;
            }, 
            Cmd::Help => {
                println!("
{} :ra  - refresh all
{} :ri - refresh instructions
{} :rf - refresh files
{} :rc - refresh converstion
{} :h  - help
{} :q  - quit",
ico_res(), ico_res(), ico_res(), ico_res(), ico_res(), ico_res());
            }
        }
    }

    // println!("\n{} buddy {} - conv {conv:?}", ico_res(), buddy.name());
    
    Ok(())
}