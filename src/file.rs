use std::{fs::File, io::Read};

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
