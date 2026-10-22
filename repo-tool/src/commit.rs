/* 
    Git helper. you know the rest lol
*/
use git2::Repository;
use dialoguer::{Input, Select};
use std::fs;

/* So a short intro to cmt_type here: 
    Basically, some repos have this rule of having a commit type in the commit message.
    For example, if you are adding a new feature, the commit message should start with [FEATURE] or [FIX] if you are fixing a bug.
    So, this cmt_type is used to add that prefix to the commit message.
*/
pub fn commit_changes(repo: &Repository, cmt_type: &str) -> Result<(), git2::Error> {
    let mut index = repo.index()?;
    index.add_all(["*"].iter(), git2::IndexAddOption::DEFAULT, None)?;
    index.write()?;

    let tree_id = index.write_tree()?;
    let tree = repo.find_tree(tree_id)?;

    let signature = repo.signature()?;
    let parent_commit = repo.head()?.peel_to_commit()?;
    // load file COMMIT_EDITMSG and clear 
    std::fs::write("./.git/COMMIT_EDITMSG", "").unwrap();

    // now we open nano to edit the commit message
    let editor = std::env::var("EDITOR").unwrap_or_else(|_| "nano".to_string());
    let status = std::process::Command::new(editor)
        .arg("./.git/COMMIT_EDITMSG")
        .status()
        .expect("Failed to open editor");

    let new_file_con = load_cmt_file();

    let commit_message = format!("{} {} \nCo-authored-by: Shourya Jangam <float314@proton.me>\n", cmt_type, new_file_con);

    // Commit on the current working branch instead of detached "HEAD".
    let head = repo.head()?;
    let branch_name = head.shorthand().unwrap_or("main");
    let refname = format!("refs/heads/{}", branch_name);

    repo.commit(
        Some(&refname),
        &signature,
        &signature,
        commit_message.as_str(),
        &tree,
        &[&parent_commit],
    )?;

    Ok(())
}

pub fn ask_for_cmt_type() -> String {
    let cmt_types = vec!["[FEATURE]", "[FIX]", "[DOCS]", "[STYLE]", "[REFACTOR]", "[TEST]", "[CHORE]", "[PERF]", "[REVERT]", "[WORKFLOW]", "[OTHER]"];
    let selection = Select::new()
        .with_prompt("Select a commit type")
        .default(0)
        .items(&cmt_types)
        .interact()
        .unwrap();

    return cmt_types[selection].to_string();
}

fn load_cmt_file() -> String {
    let file_cmt = std::fs::read_to_string("./.git/COMMIT_EDITMSG").unwrap_or_default();
    return file_cmt;
}