use anyhow::{anyhow, Context, Result};
use clap::Parser;
use git2;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long)]
    path: std::path::PathBuf,
}

fn run(args: &Cli) -> Result<()> {
    let repo = git2::Repository::open(&args.path)
        .with_context(|| format!("could not open repo `{}`", args.path.display()))?;

    if repo.is_bare() {
        return Err(anyhow!("cannot report status on bare repository"));
    }

    let mut opts = git2::StatusOptions::new();
    opts.include_ignored(false);
    opts.include_untracked(false);
    opts.exclude_submodules(true);

    let statuses = repo.statuses(Some(&mut opts))?;

    notes::print_long(&statuses);

    Ok(())
}

fn main() -> Result<()> {
    let args = Cli::parse();

    run(&args)?;
    Ok(())
}

#[test]
fn verify_cli() {
    use clap::CommandFactory;
    Cli::command().debug_assert()
}
