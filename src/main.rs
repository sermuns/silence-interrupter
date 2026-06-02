mod app;
mod audio;

use crate::app::App;

fn main() -> simple_eyre::Result<()> {
    simple_eyre::install()?;

    let mut app = App::new()?;

    app.run()?;
    // ratatui::run(|terminal| app.run(terminal))?;

    Ok(())
}
