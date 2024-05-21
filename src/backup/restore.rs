use rustic_backend::BackendOptions;
use rustic_core::{LocalDestination, LsOptions, Repository, RepositoryOptions, RestoreOptions};
use std::error::Error;

pub fn restore(
    repository: &str,
    password: &str,
    snap_path: &str,
    restore_destination: &str,
) -> Result<(), Box<dyn Error>> {
    // Initialize Backends
    let backends = BackendOptions::default()
        .repository(repository)
        .to_backends()?;

    // Open repository
    let repo_opts = RepositoryOptions::default().password(password);
    let repo = Repository::new(&repo_opts, backends)?
        .open()?
        .to_indexed()?;

    // use latest snapshot without filtering snapshots
    let node = repo.node_from_snapshot_path(snap_path, |_| true)?;

    // use list of the snapshot contents using no additional filtering
    let streamer_opts = LsOptions::default();
    let ls = repo.ls(&node, &streamer_opts)?;

    let destination = restore_destination; // restore to this destination dir
    let create = true; // create destination dir, if it doesn't exist
    let dest = LocalDestination::new(destination, create, !node.is_dir())?;

    let opts = RestoreOptions::default();
    let dry_run = false;
    // create restore infos. Note: this also already creates needed dirs in the destination
    let restore_infos = repo.prepare_restore(&opts, ls.clone(), &dest, dry_run)?;

    repo.restore(restore_infos, &opts, ls, &dest)?;
    Ok(())
}
