use git2::{build::RepoBuilder, FetchOptions, ProxyOptions};
use std::env;
use tempfile::TempDir;

#[test]
fn clone_wally_index() {
    let tmp_dir = TempDir::new().expect("create temp dir");
    let url = "https://github.com/UpliftGames/wally-index";
    // Configure proxy explicitly if one is provided via environment
    let mut fo = FetchOptions::new();
    if let Ok(proxy) = env::var("HTTPS_PROXY")
        .or_else(|_| env::var("https_proxy"))
    {
        let mut po = ProxyOptions::new();
        po.url(&proxy);
        fo.proxy_options(po);
    }

    let mut builder = RepoBuilder::new();
    builder.fetch_options(fo);
    if let Err(e) = builder.clone(url, tmp_dir.path()) {
        panic!("Failed to clone: {}", e);
    }
}
