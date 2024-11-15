use std::{thread, time::Duration};

use console::{style, Term};
use dialoguer::{theme::ColorfulTheme, Select};
use tkpr::WorkManager;
use console::Style;

fn main() {
    let term = Term::stdout();
    term.hide_cursor();
    term.clear_screen();
    let mut work_manager = WorkManager::new(None);

    work_manager.new_session("123".into(), "SAWO".into());
    work_manager.new_session("124".into(), "SAWO".into());
    work_manager.new_session("125".into(), "SAWO".into());
    work_manager.new_session("126".into(), "SAWO".into());

    thread::sleep(Duration::from_secs(5));

    let orange = Style::new().yellow();
    term.write_line("Welcome to");
    term.write_line(
        &orange.apply_to(r#"  _______ _                _  __                         
 |__   __(_)              | |/ /                         
    | |   _ _ __ ___   ___| ' / ___  ___ _ __   ___ _ __ 
    | |  | | '_ ` _ \ / _ \  < / _ \/ _ \ '_ \ / _ \ '__|
    | |  | | | | | | |  __/ . \  __/  __/ |_) |  __/ |   
    |_|  |_|_| |_| |_|\___|_|\_\___|\___| .__/ \___|_|   
                                        | |              
                                        |_|              "#).to_string()
    );

    let sessions = work_manager.get_active_sessions();
    if !sessions.is_empty() {
        term.write_line("Active Sessions:");
        for session in sessions {
            term.write_line(&format!("{}: {} -- {}", session.identifier, session.project, session.current_duration()));
        }
    } else {
        term.write_line(&format!("You currently have {} active sessions.", style("NO").magenta()));
    }

    loop {
        let selection = Select::with_theme(&ColorfulTheme::default()).with_prompt("What would you like to do?").default(0).item("End Session").item("End Session Without Saving").item("Exit").interact().unwrap();
    }

}
