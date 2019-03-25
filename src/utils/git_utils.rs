extern crate git2;
use git2::Repository;

/** 
* Gets a repo
*/
pub fn fetch(provider_url: &str ,repo_url: &str) -> Repository{
    let repository = match Repository::clone(provider_url, repo_url) {
        Ok(repo) => repo,
        Err(err) => panic!("{:?}", err)
    };

    repository
}