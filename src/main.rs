pub mod compress;

use std::{fs::{self, File}, os::unix::process::CommandExt, path::Path, process};

use serde::Deserialize;
#[derive(Deserialize)]
struct Config{
    shell:Vec<String>,
    targets:Vec<Target>
}
#[derive(Deserialize)]
struct Target{
    name:String,
    path:Vec<String>
}



fn main() {
    let config:Config = toml::from_str(&fs::read_to_string("./release.toml").unwrap()).unwrap();
    let out_put_dir = Path::new("./output");
    if !out_put_dir.exists(){
        fs::create_dir(out_put_dir);
    }else{
        for file in fs::read_dir(out_put_dir).unwrap(){
            let file = file.unwrap();
            if file.file_type().unwrap().is_dir(){
                fs::remove_dir_all(file.path()).unwrap();
            }else{
                fs::remove_file(file.path()).unwrap();
            }
        }
    }
    for shell in config.shell.iter(){
        if shell.is_empty(){continue;}
        let mut split = shell.split(' ');
        let mut commend =process::Command::new(split.next().unwrap());
        for arg in split {
            commend.arg(arg);
        }
        let mut wait =commend.spawn().unwrap();
        wait.wait().unwrap();
    }
    for target in config.targets.iter(){
         let mut target_dir = out_put_dir.to_path_buf();
        target_dir.push(&target.name);
         if !target_dir.exists(){
             fs::create_dir(&target_dir).unwrap();
         }
         for build_file in target.path.iter(){
             let build_file = Path::new(build_file);
             let mut target_file_dir = target_dir.clone();
             target_file_dir.push(build_file.file_name().unwrap());
             fs::copy(build_file, target_file_dir).unwrap();
         }
        let mut zip_file = out_put_dir.to_path_buf();
        zip_file.push(target.name.clone() +".zip");
        compress::compress_dir(target_dir.as_path(),&zip_file);
    }
    println!("done");
}
