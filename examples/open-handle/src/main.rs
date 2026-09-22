use std::{
    fs::File,
    io::{self, Read},
    path::Path,
};

fn read_opened(file: &mut File) -> io::Result<String> {
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn main() -> io::Result<()> {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("hello.txt");
    let mut file = File::open(&path)?;
    let contents = read_opened(&mut file)?;
    println!("{contents} at {}", path.display());
    Ok(())
}
