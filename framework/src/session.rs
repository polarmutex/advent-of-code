fn read_session_cookie(session_file: Option<String>) -> String {
    let path = if let Some(file) = session_file {
        PathBuf::from(file)
    } else if let Some(dir) = home_dir() {
        dir.join(SESSION_COOKIE_FILE)
    } else {
        eprintln!("error: Failed to find home directory.");
        exit(2);
    };

    match read_to_string(&path) {
        Ok(cookie) => {
            eprintln!("Loaded session cookie from \"{}\".", path.display());
            cookie
        }
        Err(err) => {
            eprintln!(
                "error: Failed to read session cookie from \"{}\": {}",
                path.display(),
                err
            );
        }
    }
}
