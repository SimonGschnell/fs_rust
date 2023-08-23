use std::{path::{Path} , fs::{read_dir}};


fn main() {
 recursive_dotfiles("./root")   
}

fn recursive_dotfiles(path:&str){

    let root = read_dir(Path::new(path)).unwrap();
    for entry in root{
        if let Ok(e) = entry{
            let filename= e.file_name();
            
            if "." == &filename.to_str().unwrap()[0..1]{
                println!("{:?}",e.path());
            }

            if let Ok(data) = e.metadata(){
                if data.is_dir(){
                    recursive_dotfiles(e.path().to_str().unwrap())
                }
            }
        }
    }
}