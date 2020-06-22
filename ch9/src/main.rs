use std::io;
use std::io::Read;
use std::io::ErrorKind;
use std::fs::File;

fn main() {
    let f = File::open("./src/hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => {
                    panic!(
                        //ファイルを作成しようとしましたが、問題がありました
                        "Tried to create file but there was a problem: {:?}",
                        e
                    )
                },
            }
        },
        Err(error) => {
            panic!(
                "There was a problem opening the file: {:?}",
                error
            )
        },
    };
    println!("{:?}", f);
    let f = File::open("./src/hello.txt").unwrap();
    println!("{:?}", f);
    let f = File::open("./src/hello.txt").expect("Fail to open file");
    println!("{:?}", f);
    let f = read_username_from_file_ver1();
    println!("{:?}", f);
    let f = read_username_from_file_ver2();
    println!("{:?}", f);
    let f = read_username_from_file();
    println!("{:?}", f);
}

fn read_username_from_file_ver1() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_ver2() -> Result<String, io::Error>{
    let f = File::open("./src/hello.txt");
    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    let mut s = String::new();
    match f.read_to_string(&mut s){
        Ok(_) =>  Ok(s),
        Err(e) =>  Err(e)
    }
}

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}