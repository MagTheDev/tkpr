use console::{style, Term};
use tkpr::WorkManager;
use console::Style;

fn main() {
    let term = Term::stdout();
    term.hide_cursor();
    term.clear_screen();
    let work_manager = WorkManager::new(None);
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
            term.write_line(&format!("{}: {}", session.identifier, session.project));
        }
    } else {
        term.write_line(&format!("You currently have {} active sessions.", style("NO").magenta()));
    }
}
