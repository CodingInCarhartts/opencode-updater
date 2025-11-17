use opencode_updater::{calculate_sha256, find_asset, verify_checksum};

#[test]
fn test_calculate_sha256() {
    let data = b"hello world";
    let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    assert_eq!(calculate_sha256(data), expected);
}

#[test]
fn test_verify_checksum() {
    let data = b"test data";
    let hash = calculate_sha256(data);
    assert!(verify_checksum(data, &hash));
    assert!(!verify_checksum(data, "invalid_hash"));
}

#[test]
fn test_find_asset() {
    let assets = vec![
        serde_json::json!({"name": "opencode-linux-x64.zip", "browser_download_url": "url1"}),
        serde_json::json!({"name": "other.zip", "browser_download_url": "url2"}),
    ];
    let found = find_asset(&assets, "opencode-linux-x64.zip");
    assert!(found.is_some());
    assert_eq!(found.unwrap()["browser_download_url"], "url1");

    let not_found = find_asset(&assets, "missing.zip");
    assert!(not_found.is_none());
}
