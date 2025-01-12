use clap::Parser;
use miv::{CliArguments, Editor};

fn main() {
    let args = CliArguments::parse();
    let mut editor = Editor::new().unwrap();

    if let Some(filename) = args.file {
        editor.editor_open_with_file(filename);
    } else {
        editor.editor_open();
    }
}
