use miv::{log, Editor};

fn main() {
    // TODO: Add VERSION to the log
    log!("Starting miv");
    let mut editor = Editor::new().unwrap();
    editor.editor_open();
}
