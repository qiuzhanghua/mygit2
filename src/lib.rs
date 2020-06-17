#[cfg(test)]
mod tests {
    #[test]
    fn open_repo() {
        use git2::Repository;

        let repo = match Repository::open(".") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };

        assert!(repo.find_remote("origin").is_ok());
    }

    #[test]
    fn test_clone_with_ssh() {
        use git2::{Cred, Error, RemoteCallbacks};
        use std::env;
        use std::path::Path;

        // Prepare callbacks.
        let mut callbacks = RemoteCallbacks::new();
        callbacks.credentials(|_url, username_from_url, _allowed_types| {
            Cred::ssh_key(
                username_from_url.unwrap(),
                None,
                std::path::Path::new(&format!("{}/.ssh/id_rsa", env::var("HOME").unwrap())),
                None,
            )
        });

        // Prepare fetch options.
        let mut fo = git2::FetchOptions::new();
        fo.remote_callbacks(callbacks);

        // Prepare builder.
        let mut builder = git2::build::RepoBuilder::new();
        builder.fetch_options(fo);

        // Clone the project.
        assert!(builder
            .clone(
                "git@github.com:qiuzhanghua/mygit2.git",
                Path::new("/tmp/mygit2"),
            )
            .is_ok());
    }

    #[test]
    fn test_fetch_origin() {
        use git2::Repository;

        let repo = match Repository::open("/tmp/mygit2") {
            Ok(repo) => repo,
            Err(e) => panic!("failed to open: {}", e),
        };
        //        assert!(repo.find_remote("origin").is_ok());
        let mut remote = repo.find_remote("origin").unwrap();
        match remote.fetch(&["master"], None, None) {
            Ok(_) => {}
            Err(e) => panic!("failed to fetch: {}", e),
            // authentication required but no callback set; class=Ssh (23)
        }
    }
}
