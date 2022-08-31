mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;

fn main() {
    // get cmd args
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // unpack file
    let journal_file = journal_file.expect("Failed to find file");

    // perform action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }.expect("Failed to perform action")
}
