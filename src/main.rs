use miv::{log, Editor};
use std::env;
fn main() {
    let mut editor = Editor::new().unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        log!("Got arguments: {:?}", args);

        let filename = args[1].clone();
        editor.editor_open_with_file(filename);
    } else {
        editor.editor_open();
    }
}
