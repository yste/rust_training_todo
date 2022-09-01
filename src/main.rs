mod cli;
mod tasks;

use structopt::StructOpt;
use cli::{Action::*, CommandLineArgs};
use tasks::Task;
use std::path::PathBuf;
use anyhow::anyhow;

fn find_default_journal() -> Option<PathBuf> {
    home::home_dir().map(|mut path| {
        path.push(".rusty-jornal.json");
        path
    })
}

fn main() -> anyhow::Result<()> {
    // get cmd args
    let CommandLineArgs {
        action,
        journal_file,
    } = CommandLineArgs::from_args();

    // unpack file
    let journal_file = journal_file.or_else(find_default_journal).ok_or(anyhow!("Failed to find journal"))?;

    // perform action
    match action {
        Add { text } => tasks::add_task(journal_file, Task::new(text)),
        List => tasks::list_tasks(journal_file),
        Done { position } => tasks::complete_task(journal_file, position),
    }?;

    Ok(())
}
