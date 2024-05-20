use rustic_backend::BackendOptions;
use rustic_core::{ConfigOptions, KeyOptions, Repository, RepositoryOptions};
use std::error::Error;

pub fn init(repository: &str, password: &str) -> Result<(), Box<dyn Error>> {
    // Initialize Backends
    let backends = BackendOptions::default()
        .repository(repository)
        .to_backends()?;

    // Init repository
    let repo_opts = RepositoryOptions::default().password(password);
    let key_opts = KeyOptions::default();
    let config_opts = ConfigOptions::default();

    if Repository::new(&repo_opts, backends.clone())?
        .open()
        .is_err()
    {
        Repository::new(&repo_opts, backends)?.init(&key_opts, &config_opts)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        let repository = "/tmp/test";
        let password = "password";

        assert!(init(repository, password).is_ok());
    }
}
