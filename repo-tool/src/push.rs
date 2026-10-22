/* 
    file to push to remote repository (GitHub, GitLab, etc.) using git2-rs library.
*/

use git2::{Cred, Error, PushOptions, RemoteCallbacks, Repository};

pub fn push_to_remote(repo: &Repository, remote_name: &str, branch_name: &str) -> Result<(), Error> {
    let mut remote = repo.find_remote(remote_name)?;

    let mut callbacks = RemoteCallbacks::new();
    callbacks.credentials(|url, username_from_url, allowed_types| {
        // Try the git credential helper first, then the SSH agent, then default.
        if allowed_types.contains(git2::CredentialType::USER_PASS_PLAINTEXT) {
            if let Ok(config) = git2::Config::open_default() {
                if let Ok(cred) = Cred::credential_helper(&config, url, username_from_url) {
                    return Ok(cred);
                }
            }
        }
        if allowed_types.contains(git2::CredentialType::SSH_KEY) {
            let user = username_from_url.unwrap_or("git");
            if let Ok(cred) = Cred::ssh_key_from_agent(user) {
                return Ok(cred);
            }
        }
        if allowed_types.contains(git2::CredentialType::DEFAULT) {
            return Cred::default();
        }
        Cred::default()
    });

    let mut options = PushOptions::new();
    options.remote_callbacks(callbacks);

    let refspec = format!("refs/heads/{}:refs/heads/{}", branch_name, branch_name);
    remote.push(&[&refspec], Some(&mut options))?;
    Ok(())
}