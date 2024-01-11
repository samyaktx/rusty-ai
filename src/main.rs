// region:    --- Modules

use textwrap::wrap;

use crate::buddy::Buddy;
use crate::utils::cli::{prompt, txt_res, ico_res, ico_err};

pub use self::ais::new_oa_client;

pub use self::error::{Error, Result};

mod error;
mod ais;
mod buddy;
mod utils;

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

// region:    --- Types

#[derive(Debug)]
enum Cmd {
    Quit,
    Chat(String),
    RefreshAll,
    RefreshConv,
    RefreshInst,
    RefreshFiles,
}

impl  Cmd {
    fn from_input(intput: impl Into<String>) -> Self {
        let input = intput.into();

        if input == ":q" {
            Self::Quit
        } else if input == ":r" || input == ":ra" {
            Self::RefreshAll
        } else if input == ":ri" {
            Self::RefreshInst
        } else if input == ":rf" {
            Self::RefreshFiles
        } else if input == ":rc" {
            Self::RefreshConv
        } else {
            Self::Chat(input)
        }
    }
}

// endregion: --- Types

const DEFAULT_DIR: &str = "buddy";

async fn start() -> Result<()> {
    let buddy = Buddy::init_from_dir(DEFAULT_DIR, false).await?;

    let conv = buddy.load_or_create_conv(false).await?;
    
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
            other => println!("{} command not supported {other:?}", ico_err()),
        }
    }

    println!("->> buddy {} - conv {conv:?}", buddy.name());
    
    Ok(())
}