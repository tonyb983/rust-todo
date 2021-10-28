use std::{io::Read, path::Path};

pub struct FileSystem;

impl FileSystem {
    /// Saves the given bytes into a file at the given path. The file will be overwritten if
    /// it already exists, and created if it does not.
    /// 
    /// ### Arguments
    /// * `file` - The path to the target file
    /// * `data` - The data to save to the file
    pub fn save_bytes<TFilePath: AsRef<Path>, TData: AsRef<[u8]>>(
        file: TFilePath,
        data: TData,
    ) -> std::io::Result<()> {
        std::fs::write(file, data)
    }

    /// Loads all bytes from the file at the given path and returns them as a [`Vec<u8>`]
    /// 
    /// ### Arguments
    /// * `file` - The path to the target file
    pub fn load_bytes<TFilePath: AsRef<Path>>(
        file: TFilePath,
    ) -> std::io::Result<Vec<u8>> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file)?;

        let mut buf: Vec<u8> = Vec::new();
        f.read_to_end(&mut buf);
        
        Ok(buf)
    }

    /// Loads all bytes from the file at the given path into the provided buffer and returns the number of bytes read.
    /// 
    /// ### Arguments
    /// * `file` - The path to the target file
    /// * `buffer` - The buffer in which to put the bytes that are read
    pub fn load_bytes_into<TFilePath: AsRef<Path>>(
        file: TFilePath,
        buffer: &mut Vec<u8>,
    ) -> std::io::Result<usize> {
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open(file)?;

        f.read_to_end(buffer)
    }
}
