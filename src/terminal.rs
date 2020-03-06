use crossterm::{
    terminal::{Clear, ClearType},
    ExecutableCommand, Result,
};

use std::io::stdout;

pub fn clear() -> Result<()> {
    stdout().execute(Clear(ClearType::All))?;

    Ok(())
}
