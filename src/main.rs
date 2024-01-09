// region:    --- Modules

pub use self::error::{Error, Result};

mod error;

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
    println!("Hello, world!");
    Ok(())
}