use std::{
    fs::{create_dir_all, write, File},
    io::Read,
    path::PathBuf,
};

use axum::body::Bytes;

pub fn write_file(file_path: &PathBuf, contents: &Bytes) -> Result<(), std::io::Error> {
    if let Some(parent) = file_path.parent() {
        create_dir_all(parent).ok();
    }

    write(file_path, &contents).ok();
    Ok(())
}

pub fn read_file(file_path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::prelude::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_write_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let contents = Bytes::from("test");

        write_file(&file_path, &contents).unwrap();

        let mut file = File::open(file_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();

        assert_eq!(file_contents, "test");
    }

    #[tokio::test]
    async fn test_write_file_to_parent() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("parent/test.txt");
        let contents = Bytes::from("test");

        write_file(&file_path, &contents).unwrap();

        let mut file = File::open(file_path).unwrap();
        let mut file_contents = String::new();
        file.read_to_string(&mut file_contents).unwrap();

        assert_eq!(file_contents, "test");
    }

    #[tokio::test]
    async fn test_read_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(file_path.clone())
            .and_then(|file| Ok(file))
            .unwrap();

        file.write_all(b"test").unwrap();

        let contents = read_file(file_path.to_str().unwrap()).unwrap();

        assert_eq!(contents, "test");
    }
}
