use git2::Repository;
use tempfile::TempDir;

#[test]
fn clone_wally_index() {
    let tmp_dir = TempDir::new().expect("create temp dir");
    let url = "https://github.com/UpliftGames/wally-index";
    match Repository::clone(url, tmp_dir.path()) {
        Ok(_) => {},
        Err(e) => panic!("Failed to clone: {}", e),
    }
}
