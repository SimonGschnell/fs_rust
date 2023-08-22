use std::{path::{Path} , fs::{read_dir}};


fn main() {
    let root = read_dir(Path::new("./root")).unwrap();
    for entry in root{
        if let Ok(e) = entry{
            let filename= e.file_name();
            print!("{:?}",filename);
            if "." == &filename.to_str().unwrap()[0..1]{
                print!(" dotfile ");
            }else{
                print!(" non dotfile ");
            }

            if let Ok(data) = e.metadata(){
                print!(" - {:?}\n",data.file_type());
            }
        }
    }
}
