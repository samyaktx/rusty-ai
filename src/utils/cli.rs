use console::{Style, style, StyledObject};
use dialoguer::{Input, theme::ColorfulTheme};

use crate::Result;

// region:       --- Prompts

pub fn prompt(text: &str) -> Result<String> {
    let theme = ColorfulTheme {
        prompt_style: Style::new().for_stderr().color256(45),
        prompt_prefix: style("?".to_string()).color256(45).for_stderr(),
        ..ColorfulTheme::default()
    };

    let input = Input::with_theme(&theme);
    let res = input.with_prompt(text).interact_text()?;

    Ok(res)
}

// endregion:    --- Prompts

// region:       --- Icons

pub fn ico_check() -> StyledObject<&'static str> {
    style("✔️").green()
}

pub fn ico_uploading() -> StyledObject<&'static str> {
    style("⇭").yellow()
}

pub fn ico_uploaded() -> StyledObject<&'static str> {
    style("⇭").green()
}

pub fn ico_deleted_ok() -> StyledObject<&'static str> {
    style("⌫").green()
}

pub fn ico_err() -> StyledObject<&'static str> {
    style("✗").green()
}

// endregion:    --- Icons

// region:       --- Text Output

pub fn txt_res(text: String) -> StyledObject<String> {
    style(text).bright()
}

// endregion:    --- Text Output
