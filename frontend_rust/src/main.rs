// frontend_rust/src/main.rs

// frontend_rust/src/main.rs

mod audio;
mod cli;
mod config;
mod gui;
mod recording;

fn main() -> Result<(), String> {
    println!("Project Aurora initializing...");

    // Initialize components (placeholders for now)
    cli::parse_arguments();
    config::load_config();
    // audio::init_audio(); // Will be handled by the GUI/main loop for now

    match gui::run_application() {
        Ok(_) => println!("Project Aurora exited cleanly."),
        Err(e) => {
            eprintln!("Application error: {}", e);
            return Err(e);
        }
    }

    Ok(())
}
