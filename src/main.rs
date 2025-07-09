use std::io;

mod app;
mod tui;
mod vpn;

use app::App;

fn main() -> io::Result<()> {
    let mut app = App::new();
    let mut terminal = tui::init()?;
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        tui::restore().expect("Failed to restore terminal on panic.");
        original_hook(panic_info);
    }));
    tui::run(&mut terminal, &mut app)?;
    tui::restore()?;
    Ok(())
}
