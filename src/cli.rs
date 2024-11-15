use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generation-related commands
    #[command(subcommand)]
    Gen(GenerationCommand),
    /// Install a package with the specified manager
    Install(InstallPkg),
    /// Removes a package with the specified manager
    Remove(RemovePkg),
    /// Sync the database of a manager
    Sync(SyncPkg),
}

#[derive(Subcommand)]
pub enum GenerationCommand {
    /// Make the changes take effect, this starts a new generation
    Build,
    /// Remove a generation
    Remove(RemoveGeneration),
    /// Rollback to previous generation
    Rollback(RollbackGeneration),
}

#[derive(Args)]
pub struct RemoveGeneration {
    /// Generation ID to remove
    genid: u32,
}

#[derive(Args)]
pub struct RollbackGeneration {
    /// Generation ID to rollback to
    genid: u32,
}

#[derive(Args)]
pub struct InstallPkg {
    /// Name of package to install
    pkg_name: String,
    /// In which package pack to install
    package_pack: String,
    /// Manager to use
    manager: String,
}

#[derive(Args)]
pub struct RemovePkg {
    /// Name of package to remove
    pkg_name: String,
    /// From which package pack to remove
    package_pack: String,
    /// Manager to use
    manager: String,
}

#[derive(Args)]
pub struct SyncPkg {
    /// Manager to sync
    manager: String,
}
