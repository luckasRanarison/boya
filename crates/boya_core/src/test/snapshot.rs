#[macro_export]
macro_rules! assert_snapshot {
    ($content: expr) => {
        use std::path::PathBuf;

        let mut settings = insta::Settings::clone_current();

        let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

        path.push("snapshots");
        settings.set_snapshot_path(path);
        settings.set_prepend_module_to_snapshot(true);

        settings.bind(|| {
            insta::assert_snapshot!($content);
        });
    };
}
