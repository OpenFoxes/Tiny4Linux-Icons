use iced::widget::image::Handle;
use once_cell::sync::Lazy;
use std::env;
use std::path::{Path, PathBuf};

static ROOT: Lazy<PathBuf> = Lazy::new(|| {
    // path root should be the crate root
    let root = env!("CARGO_MANIFEST_DIR");
    Path::new(root).to_path_buf()
});

pub fn absolute_path_for_t4l_asset<P: AsRef<Path>>(relative_path: P) -> PathBuf {
    ROOT.join(relative_path)
}

pub fn handle_t4l_asset<P: AsRef<Path>>(relative_path: P) -> Handle {
    let abs_path = absolute_path_for_t4l_asset(relative_path);
    Handle::from_path(abs_path)
}
