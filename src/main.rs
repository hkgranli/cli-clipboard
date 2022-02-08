use std::fmt; // import format
use std::fs; // import file system


use clipboard::ClipboardProvider; // read and write clipboard
use clipboard::ClipboardContext; 

use std::io;
use std::io::Write;

// easily fetch home-directory
use home::home_dir;
use std::fs::File;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
struct Clip{
    name : String,
    code : String
}

impl fmt::Display for Clip{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
        write!(fmt, "{}, {}", self.name, self.code)
    }
}
/*
impl fmt::Debug for Clip{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
        write!(fmt, "{}, {}", self.name, self.code)
    }
}
*/
fn create_clip(name: String, code: String) -> Clip{
    Clip{
        name, 
        code
    }
}

fn file_exists(path : &String) -> bool{
    return fs::metadata(path).is_ok()
}

fn read_clips(path : &String) -> Vec<Clip>{

    let content: String; 
    let clip_vec : Vec<Clip>;

    if file_exists(&path){
        content = fs::read_to_string(&path).expect("Something went wrong reading the file");

        clip_vec = match serde_json::from_str(&content) {
            Ok(val)  => val,
            Err(_err) => Vec::new(),
        };

    }else{
        clip_vec = Vec::new();
         //empty json
        File::create(&path)
    .expect("failed to create clipboardfile");
    }

    // use serde to convert the json string to vec 

    return clip_vec;

}

fn write_clips(clips : Vec<Clip>, path:&String) -> (){
    fs::remove_file(&path)
    .expect("Something went wrong deleting the file");

    // serialize the vector into json string 
    let clip_string = serde_json::to_string(&clips).unwrap();

    match fs::write(&path, clip_string){
        Ok(_v) => println!("Saved successfully"),
        Err(_v) => println!("Error saving")
    }

}

fn remove_clip(index: usize, mut group: Vec<Clip>) -> Vec<Clip>{
    group.remove(index);
    return group;
}

fn read_from_clipboard() -> String{
    let mut ctx : ClipboardContext = ClipboardProvider::new().unwrap();
    
    let val = ctx.get_contents();

    match val {
        Ok(v) => return v.to_string(),
        Err(_e) => panic!("couldnt read clipboard")
    }
}

fn get_help() -> (){
    let output = "cpd commands:
    \t- add/new: store a new Clip => ex: cpd add
    \t- remove/rm/delete + [index]: removes a stored Clip by index => ex: cpd remove 2
    \t- list/ls: lists all Clips => ex: cpd list
    \t\t flags: -v | --verbose => prints both keyword and code
    \t- get + [index]: copies desired Clip code to clipboard => ex: cpd get 1
    ".to_string();

    println!("{}", output);
}

fn add_clip(mut vecman : Vec<Clip>){
    let mut name = String::new();
    
    println!("What's the name of the Clip?");
    let _ = io::stdout().flush();
    let _ = io::stdin().read_line(&mut name);

    let new_clip = create_clip(name.replace("\n", ""), read_from_clipboard());

    vecman.push(new_clip);

    let path: String = find_clipfile();

    write_clips(vecman, &path.to_string());
}

fn find_clipfile() -> String {
    
    let home = home_dir();

    match home{
        Some(v) => {
            let path = v.into_os_string().into_string().unwrap();
            let path_string = format!("{}/.clipboardfile", path);
            return path_string;
        },
        None => panic!("Home-directory not found")
    }
}

fn list_clips(vecman : &Vec<Clip>){
    for (index, clip) in vecman.iter().enumerate(){
        println!("{} {}", index+1, clip.name);
    }
}

fn list_clips_verbose(vecman : &Vec<Clip>){
    for (index, clip) in vecman.iter().enumerate(){
        println!("{} {} -> {}", index+1, clip.name, clip.code);
    }
}

fn get_clip(vecman : &Vec<Clip>){
    let index_input = std::env::args().nth(2).expect("No further argument given, use 'cpd list' or 'cpd help'");
    let index : usize = index_input.parse().unwrap();
    if index-1 > vecman.len(){
        panic!("Invalid index!! cpd list for valid arguments");
    }
    println!("Copying entry for {} to clipboard", vecman[index-1].name);
    write_to_clipboard(&vecman[index-1].code.to_string());
}

fn write_to_clipboard(cont : &String) -> (){
    let mut ctx : ClipboardContext = ClipboardProvider::new().unwrap();
    
    match  ctx.set_contents(cont.to_string()){
        Ok(_v) => println!("wrote {} to clipboard", cont),
        Err(e) => println!("{}",e)
    }
    let _ = ctx.get_contents();
}

fn remove_clip_cli(vecman : Vec<Clip>){
    let index_input = std::env::args().nth(2).expect("No further argument given, use 'cpd list' or 'Cliptab help'");
    let index : usize = index_input.parse().unwrap();
    if index-1 > vecman.len(){
        println!("Invalid index!!");
        list_clips(&vecman);
        std::process::exit(0);
    }
    println!("Removing content of {}", vecman[index-1].name);
    let new_vec : Vec<Clip> = remove_clip(index-1, vecman);

    let path : String = find_clipfile();

    write_clips(new_vec, &path);

}

fn main() {

    let action = std::env::args().nth(1).expect("No argument given, try 'cpd help' for commands");

    let vecman : Vec<Clip> = read_clips(&find_clipfile());

    match action.as_str(){
        "add" | "new" => {
            add_clip(vecman);
        },
        "remove" | "rm" | "delete" => {
            remove_clip_cli(vecman);
        },
        "get" => {
            get_clip(&vecman);
        },
        "list" | "ls" => {
            match std::env::args().nth(2){
                Some(val) => match val.as_str(){
                    "--verbose" | "-v" => {
                        list_clips_verbose(&vecman);
                    },
                    _ => {
                        list_clips(&vecman);
                    }
                },
                None => {
                    list_clips(&vecman);
                }
            }
            
        },
        _  => {
            get_help();
        }
    }
}
