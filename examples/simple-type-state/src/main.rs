use std::{
    io,
    path::{Path, PathBuf},
};

struct FileNotOpened;
struct FileOpened;

// Sets default to `FileNotOpened`
#[derive(Debug)]
struct File<State = FileNotOpened> {
    path: PathBuf,
    handle: Option<std::fs::File>,
    _state: std::marker::PhantomData<State>,
}

impl File<FileNotOpened> {
    fn open(path: &Path) -> io::Result<File<FileOpened>> {
        let file = std::fs::File::open(path)?;
        Ok(File {
            path: path.to_path_buf(),
            handle: Some(file),
            _state: std::marker::PhantomData::<FileOpened>,
        })
    }
}

impl File<FileOpened> {
    fn read(&mut self) -> io::Result<String> {
        use io::Read;

        let mut content = String::new();
        let Some(handle) = self.handle.as_mut() else {
            unreachable!("Safe to unwrao as state can only be reached when file is open");
        };
        handle.read_to_string(&mut content)?;
        Ok(content)
    }

    const fn path(&self) -> &PathBuf {
        &self.path
    }
}

fn main() {
    let dir = std::env::current_dir().unwrap();
    let path = dir.join("examples/simple-type-state/hello.txt");
    let mut file = File::open(&path).unwrap();
    let content = file.read().unwrap();

    println!("{content} at {}", file.path().display());
}
