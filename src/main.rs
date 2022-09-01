mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;

fn find_default_journal() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-jornal.json");
        path
    })
}

fn main() {
    // get cmd args
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // unpack file
    let journal_file = journal_file.or_else(find_default_journal).expect("Failed to find file");

    // perform action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }.expect("Failed to perform action")
}
