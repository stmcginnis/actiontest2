use migration_helpers::common_migrations::AddSettingsMigration;
use migration_helpers::{migrate, Result};
use std::process;

/// We added a new settings for configuring kubelet, `settings.kubernetes.container-log-max-size`
/// and `settings.kubernetes.container-log-max-files`
fn run() -> Result<()> {
    migrate(AddSettingsMigration(&[
        "settings.kubernetes.container-log-max-size",
        "settings.kubernetes.container-log-max-files",
    ]))
}

// Returning a Result from main makes it print a Debug representation of the error, but with Snafu
// we have nice Display representations of the error, so we wrap "main" (run) and print any error.
// https://github.com/shepmaster/snafu/issues/110
fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        process::exit(1);
    }
}
