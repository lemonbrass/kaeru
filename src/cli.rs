use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Generation-related commands
    #[command(subcommand)]
    Gen(GenerationCommand),
    /// Install a package with the specified manager
    Install(PkgData),
    /// Removes a package with the specified manager
    Remove(PkgData),
    /// Sync the database of a manager
    Sync(SyncPkg),
}

#[derive(Subcommand)]
pub enum GenerationCommand {
    /// Remove a generation
    Remove(GenerationId),
    /// Remove duplicate generations
    RemoveDuplicates,
    /// Rollback to a generation id.
    Rollback(GenerationId),
    /// List all generations
    List,
    /// Make the changes take effect, this starts a new generation
    Commit(GenerationMessage),
}

#[derive(Args)]
pub struct GenerationId {
    /// Generation ID, find it using kaeru gen list
    pub genid: usize,
}

#[derive(Args)]
pub struct GenerationMessage {
    /// Commit message for the generation
    pub genmsg: String,
}

#[derive(Args)]
pub struct PkgData {
    /// In which package pack to install
    pub package_pack: String,
    /// Name of package to install
    pub pkg_names: Vec<String>,
}

#[derive(Args)]
pub struct SyncPkg {
    /// Manager to sync
    pub manager: String,
}
