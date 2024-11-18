pub const ASK_FOR_SETUP_MSG: &str = "Kaeru isn't setup right now, want to run setup? [Y/n]: ";
pub const ERR_INVALID_GENID: &str =
    "Specified generation ID is invalid, use kaeru gen list to list all generations";
pub const SETUP_COMPLETE: &str = "Setup finished, run `kaeru help` to get started.";
pub const ERR_NO_CHANGES_TO_COMMIT: &str = "No changes to commit, cannot create generation.";
pub const DEFAULT_CONFIG: &str = r#"
[managers]
# call_order = ["manager1", "manager2", "manager3"]

[packages]
# package_order = ["essentials", "dev", "games"]
# These will be installed in the order provided.
# The unspecified ones will be installed after these.
"#;
