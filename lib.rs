// This function print out an output similar to git's status command in long
// form, including the command-line hints.
fn print_long(statuses: &git2::Statuses) {
    let mut header = false;
    let mut rm_in_workdir = false;
    let mut changes_in_index = false;
    let mut changed_in_workdir = false;

    // Print index changes
    for entry in statuses
        .iter()
        .filter(|e| e.status() != git2::Status::CURRENT)
    {
        if entry.status().contains(git2::Status::WT_DELETED) {
            rm_in_workdir = true;
        }
        let istatus = match entry.status() {
            s if s.contains(git2::Status::INDEX_NEW) => "new file: ",
            s if s.contains(git2::Status::INDEX_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::INDEX_DELETED) => "deleted: ",
            s if s.contains(git2::Status::INDEX_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::INDEX_TYPECHANGE) => "typechange:",
            _ => continue,
        };
        if !header {
            println!(
                "\
# Changes to be committed:
#   (use \"git reset HEAD <file>...\" to unstage)
#"
            );
            header = true;
        }

        let old_path = entry.head_to_index().unwrap().old_file().path();
        let new_path = entry.head_to_index().unwrap().new_file().path();
        match (old_path, new_path) {
            (Some(old), Some(new)) if old != new => {
                println!("#\t{}  {} -> {}", istatus, old.display(), new.display());
            }
            (old, new) => {
                println!("#\t{}  {}", istatus, old.or(new).unwrap().display());
            }
        }
    }

    if header {
        changes_in_index = true;
        println!("#");
    }
    header = false;

    // Print workdir changes to tracked files
    for entry in statuses.iter() {
        // With `Status::OPT_INCLUDE_UNMODIFIED` (not used in this example)
        // `index_to_workdir` may not be `None` even if there are no differences,
        // in which case it will be a `Delta::Unmodified`.
        if entry.status() == git2::Status::CURRENT || entry.index_to_workdir().is_none() {
            continue;
        }

        let istatus = match entry.status() {
            s if s.contains(git2::Status::WT_MODIFIED) => "modified: ",
            s if s.contains(git2::Status::WT_DELETED) => "deleted: ",
            s if s.contains(git2::Status::WT_RENAMED) => "renamed: ",
            s if s.contains(git2::Status::WT_TYPECHANGE) => "typechange:",
            _ => continue,
        };

        if !header {
            println!(
                "\
# Changes not staged for commit:
#   (use \"git add{} <file>...\" to update what will be committed)
#   (use \"git checkout -- <file>...\" to discard changes in working directory)
#\
                ",
                if rm_in_workdir { "/rm" } else { "" }
            );
            header = true;
        }

        let old_path = entry.index_to_workdir().unwrap().old_file().path();
        let new_path = entry.index_to_workdir().unwrap().new_file().path();
        match (old_path, new_path) {
            (Some(old), Some(new)) if old != new => {
                println!("#\t{}  {} -> {}", istatus, old.display(), new.display());
            }
            (old, new) => {
                println!("#\t{}  {}", istatus, old.or(new).unwrap().display());
            }
        }
    }

    if header {
        changed_in_workdir = true;
        println!("#");
    }
    header = false;

    // Print untracked files
    for entry in statuses
        .iter()
        .filter(|e| e.status() == git2::Status::WT_NEW)
    {
        if !header {
            println!(
                "\
# Untracked files
#   (use \"git add <file>...\" to include in what will be committed)
#"
            );
            header = true;
        }
        let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
        println!("#\t{}", file.display());
    }
    header = false;

    // Print ignored files
    for entry in statuses
        .iter()
        .filter(|e| e.status() == git2::Status::IGNORED)
    {
        if !header {
            println!(
                "\
# Ignored files
#   (use \"git add -f <file>...\" to include in what will be committed)
#"
            );
            header = true;
        }
        let file = entry.index_to_workdir().unwrap().old_file().path().unwrap();
        println!("#\t{}", file.display());
    }

    if !changes_in_index && changed_in_workdir {
        println!(
            "no changes added to commit (use \"git add\" and/or \
             \"git commit -a\")"
        );
    }
}
