use miv::{log, Editor};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        log!("Got arguments: {:?}", args);

        let filename = args[1].clone();
        let mut editor = Editor::new().unwrap();
        editor.editor_open_with_file(filename);
    } else {
        let mut editor = Editor::new().unwrap();
        editor.editor_open();
    }
}
