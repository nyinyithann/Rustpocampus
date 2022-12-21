// ANCHOR: all
use std::fs::File;
use std::io::Read;
use std::io;
use std::io::ErrorKind;

//  using panic!
fn read_file(path: &str) -> io::Result<()> {
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(error) => panic!("Error: {:?}", error),
    };
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("{:?}", buf);
    Ok(())
}

// using panic!
fn read_or_create_file(path: &str) -> io::Result<()> {
    let mut file = File::open(path).unwrap_or_else(|error|{
       if error.kind() == ErrorKind::NotFound {
            File::create(path).unwrap_or_else(|error| {
                panic!("Error creating file {:?}", error);
            })
        } else {
            panic!("Error opening file");
        }
    });
    let mut buf = String::new();
    file.read_to_string(&mut buf)?;
    println!("{:?}", buf);
    Ok(())
}

// using Result propagation operator ?
fn read_content(path: &str) -> io::Result<String> {
    let mut buf = String::new();
    let mut f = File::open(path)?;
    File::read_to_string(&mut f,&mut buf)?;
    Ok(buf)
}

fn main() -> io::Result<()> {
    read_file("hello.txt")?;
    read_or_create_file("abc.txt")?;
    println!("{:?}", read_content("hello.txt")?);
    Ok(())
}
// ANCHOR_END: all
