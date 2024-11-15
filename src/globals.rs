pub const ASK_FOR_SETUP_MSG: &str = "Kaeru isn't setup right now, want to run setup? [Y/n]: ";
pub const DEFAULT_CONFIG: &str = r#"
[managers]
# call_order = ["manager1", "manager2", "manager3"]

[packages]
# package_order = ["essentials", "dev", "games"]
# These will be installed in the order provided.
# The unspecified ones will be installed after these.
"#;
