use std::{path::{Path, PathBuf} , fs::{read_dir}, slice::SliceIndex};
use std::env::{args_os,ArgsOs};
use std::ffi::OsString;

fn main() {
    let mut args = args_os();
    
 recursive_dotfiles(args.nth(1).unwrap_or(OsString::from("./example"))); 
}

fn recursive_dotfiles(path:OsString){

    let root = read_dir(PathBuf::from(path).as_path()).unwrap();
    for entry in root{
        if let Ok(e) = entry{
            let filename= e.file_name();
            let file_str = filename.to_str().unwrap_or("placeholder");
            
                if let Some(first_letter)=file_str.get(0..1){
                    if "." == first_letter{
                        println!("{:?}",e.path());
                    }
                }
                
           
            

            if let Ok(data) = e.metadata(){
                if data.is_dir(){
                    recursive_dotfiles(e.path().into_os_string())
                }
            }
        }
    }
}