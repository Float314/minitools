/* 
    Git-helper: Personal helper. 

    This is a personal CLI Tool to add Co-authors (well, myself using second email lol) and all. 
    Also used to Push repositories to GitHub and GitLab (Wherever Origin is set to). 
*/
// include all the created files
mod cli;
mod commit;
mod push;
use git2::Repository;
use dialoguer::Confirm;
use std::process::Command;

fn main() {
    let choice = cli::ask();

    if choice == "Commit" { 
        commit::commit_changes(
            &Repository::open(".").unwrap(),
            &commit::ask_for_cmt_type()
        ).unwrap();
        let msg2 = Confirm::new()
            .with_prompt("Do you want to push the changes?")
            .interact()
            .unwrap();

        if msg2 { 
            println!("Pushing changes to remote...");
            let repo = Repository::open(".").unwrap();
            let branch = repo.head().unwrap().shorthand().unwrap_or("main").to_string();
            push::push_to_remote(
                &repo,
                "origin",
                &branch
            ).unwrap();
        }
        print!("commit successful!\n");
        Command::new("git")
            .args(["log", "-1", "--pretty=format:%H"])
            .output()
            .expect("Failed to execute command");


    } else if choice == "Push" { 
        let repo = Repository::open(".").unwrap();
        let branch = repo.head().unwrap().shorthand().unwrap_or("main").to_string();
        push::push_to_remote(
            &repo,
            "origin",
            &branch
        ).unwrap();
    } else { 
        std::process::exit(-1);
    }

    std::process::exit(0);
}
