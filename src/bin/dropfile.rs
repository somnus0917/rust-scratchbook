fn main() {
    println!("hello dropfile");
    {
        let f = Tempfile::new(PathBuf::from("test.txt"));
    }
    let temp2 = Tempfile::new(PathBuf::from("test2.txt"));
    drop(temp2);
    println!("手动丢弃");
}
use std::fs::File;
use std::path::PathBuf;

struct Tempfile {
    file: File,
    filepath: PathBuf,
}
impl Tempfile {
    fn new(path: PathBuf) -> std::io::Result<Self> {
        let file = File::create(&path)?;
        Ok(Self {
            file,
            filepath: path,
        })
    }
}
impl Drop for Tempfile {
    fn drop(&mut self) {
        if let Err(e) = std::fs::remove_file(&self.filepath) {
            eprintln!("delete failed {}", e)
        }
        println!("succeed to delete file {:?}", self.filepath)
    }
}
