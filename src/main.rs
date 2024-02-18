// region:    --- Modules

use textwrap::wrap;

use crate::rusty_ai::RustyAI;
use crate::utils::cli::{prompt, txt_res, ico_res};
pub use self::ais::new_oa_client;
pub use self::error::{Error, Result};

mod error;
mod ais;
mod rusty_ai;
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

const DEFAULT_DIR: &str = "rusty_ai";

async fn start() -> Result<()> {
    let mut rusty_ai = RustyAI::init_from_dir(DEFAULT_DIR, false).await?;

    let mut conv = rusty_ai.load_or_create_conv(false).await?;
    
    loop {
        println!();
        let input = prompt("rusty-ai query")?;
        let cmd = Cmd::from_input(input);

        match cmd {
            Cmd::Quit => break,
            Cmd::Chat(msg) => {
                let res = rusty_ai.chat(&conv, &msg).await?;
                let res = wrap(&res, 80).join("\n");
                println!("{} {}",  ico_res(), txt_res(res));
            },
            Cmd::RefreshAll => {
                rusty_ai = RustyAI::init_from_dir(DEFAULT_DIR, true).await?;
                conv = rusty_ai.load_or_create_conv(true).await?;
            },
            Cmd::RefreshConv => {
                conv = rusty_ai.load_or_create_conv(true).await?;
            },
            Cmd::RefreshInst => {
                rusty_ai.upload_instructions().await?;
                conv = rusty_ai.load_or_create_conv(true).await?;
            },
            Cmd::RefreshFiles => {
                rusty_ai.upload_files(true).await?;
                conv = rusty_ai.load_or_create_conv(true).await?;
            }, 
            Cmd::Help => {
                println!(
                    "{} :ra - refresh all\n{} :ri - refresh instructions\n{} :rf - refresh files\n{} :rc - refresh converstion\n{} :h  - help\n{} :q  - quit",
                    ico_res(), ico_res(), ico_res(), ico_res(), ico_res(), ico_res()
                );
            }
        }
    }

    // println!("\n{} rusty_ai {} - conv {conv:?}", ico_res(), rusty_ai.name());
    
    Ok(())
}