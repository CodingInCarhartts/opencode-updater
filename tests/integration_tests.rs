use opencode_updater::{
    Args, calculate_sha256, download_with_progress, find_asset, find_executable_binary, run_update,
    verify_checksum,
};
use std::io::Cursor;

/// Test SHA-256 hash calculation.
#[test]
fn test_calculate_sha256() {
    let data = b"hello world";
    let expected = "b94d27b9934d3e08a52e52d7da7dabfac484efe37a5380ee9088f7ace2efcde9";
    assert_eq!(calculate_sha256(data), expected);
}

/// Test checksum verification.
#[test]
fn test_verify_checksum() {
    let data = b"test data";
    let hash = calculate_sha256(data);
    assert!(verify_checksum(data, &hash));
    assert!(!verify_checksum(data, "invalid_hash"));
}

/// Test finding an asset by name.
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

/// Test locating an executable binary in a directory.
#[test]
fn test_find_executable_binary() {
    use std::fs::File;
    use std::io::Write;
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    let temp_dir = tempdir().unwrap();
    let exec_path = temp_dir.path().join("executable");
    let mut exec_file = File::create(&exec_path).unwrap();
    exec_file.write_all(b"binary content").unwrap();
    drop(exec_file);

    let mut perms = exec_path.metadata().unwrap().permissions();
    perms.set_mode(0o755);
    std::fs::set_permissions(&exec_path, perms).unwrap();

    // Create a non-executable file
    let non_exec_path = temp_dir.path().join("non_executable");
    File::create(&non_exec_path).unwrap();

    let result = find_executable_binary(temp_dir.path());
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), exec_path);

    // Test with no executable
    let empty_dir = tempdir().unwrap();
    let result_empty = find_executable_binary(empty_dir.path());
    assert!(result_empty.is_err());
}

/// Integration test for the update process with mocked network calls.
#[test]
fn test_run_update() {
    use std::io::Write;
    use zip::write::ZipWriter;

    // Create a mock zip with a file
    let mut zip_buffer = Vec::new();
    {
        let mut zip = ZipWriter::new(Cursor::new(&mut zip_buffer));
        zip.start_file(
            "opencode",
            zip::write::FileOptions::<()>::default().unix_permissions(0o755),
        )
        .unwrap();
        zip.write_all(b"fake binary content").unwrap();
        zip.finish().unwrap();
    }
    let zip_bytes = zip_buffer;

    // Calculate checksum for the zip
    let checksum = calculate_sha256(&zip_bytes);

    // Start mock server
    let mut server = mockito::Server::new();
    let url = server.url();

    // Mock the release API
    let release_mock = server
        .mock("GET", "/repos/sst/opencode/releases/latest")
        .with_status(200)
        .with_header("content-type", "application/json")
        .with_body(format!(
            r#"{{
            "assets": [
                {{
                    "name": "opencode-linux-x64.zip",
                    "browser_download_url": "{}/download/zip"
                }},
                {{
                    "name": "opencode-linux-x64.zip.sha256",
                    "browser_download_url": "{}/download/sha256"
                }}
            ]
        }}"#,
            url, url
        ))
        .create();

    // Mock the zip download
    let zip_mock = server
        .mock("GET", "/download/zip")
        .with_status(200)
        .with_body(zip_bytes)
        .create();

    // Mock the checksum download
    let checksum_mock = server
        .mock("GET", "/download/sha256")
        .with_status(200)
        .with_body(&checksum)
        .create();

    // Create client and args
    let client = ureq::Agent::new();
    let args = Args { bin: false }; // Use default asset

    // Run the update process with mocks
    let asset_override = Some((
        "opencode-linux-x64.zip".to_string(),
        format!("{}/download/zip", url),
    ));
    let result = run_update(&args, &client, &url, asset_override, true);
    assert!(result.is_ok());

    // Verify mocks were called
    release_mock.assert();
    zip_mock.assert();
    checksum_mock.assert();
}

/// Test the download_with_progress function with mocked HTTP response.
#[test]
fn test_download_with_progress() {
    // Create test data
    let test_data = b"Hello, World! This is test data for download progress.";
    let data_size = test_data.len() as u64;

    // Start mock server
    let mut server = mockito::Server::new();
    let url = server.url();

    // Mock the download endpoint with content-length header
    let download_mock = server
        .mock("GET", "/test-download")
        .with_status(200)
        .with_header("content-length", &data_size.to_string())
        .with_body(test_data)
        .create();

    // Test the download function
    let client = ureq::Agent::new();
    let download_url = format!("{}/test-download", url);
    let result = download_with_progress(&client, &download_url, "test-file.txt");

    assert!(result.is_ok());
    let downloaded_data = result.unwrap();
    assert_eq!(downloaded_data, test_data);

    // Verify mock was called
    download_mock.assert();
}

/// Test the download_with_progress function without content-length header.
#[test]
fn test_download_with_progress_no_content_length() {
    let test_data = b"Test data without content-length header";

    // Start mock server
    let mut server = mockito::Server::new();
    let url = server.url();

    // Mock the download endpoint without content-length header
    let download_mock = server
        .mock("GET", "/test-download-no-length")
        .with_status(200)
        .with_body(test_data)
        .create();

    // Test the download function
    let client = ureq::Agent::new();
    let download_url = format!("{}/test-download-no-length", url);
    let result = download_with_progress(&client, &download_url, "test-file-no-length.txt");

    assert!(result.is_ok());
    let downloaded_data = result.unwrap();
    assert_eq!(downloaded_data, test_data);

    // Verify mock was called
    download_mock.assert();
}
