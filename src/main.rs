extern crate clipboard;

use std::fmt; // import format
use std::fs; // import file system


use clipboard::ClipboardProvider; // read and write clipboard
use clipboard::ClipboardContext; 

use std::io;
use std::io::Write;

use home::home_dir;

struct Clip{
    name : String,
    code : String
}

impl fmt::Display for Clip{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
        write!(fmt, "{}, {}", self.name, self.code)
    }
}

impl fmt::Debug for Clip{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result{
        write!(fmt, "{}, {}", self.name, self.code)
    }
}

fn create_clip(name: String, code: String) -> Clip{
    Clip{
        name, 
        code
    }
}

fn read_clips(path : &String) -> Vec<Clip>{
    let mut clip_vec = Vec::new();
    
    let content = fs::read_to_string(&path)
    .expect("Something went wrong reading the file");

    let mut split = content.lines();

    for clip_str in split{
        let values : Vec<&str> = clip_str.split(",").collect();
        if values.len() != 2 {
            continue;
        }
        clip_vec.push(create_clip(values[0].to_string(), values[1].to_string()));
    }

    return clip_vec;

}

fn write_clips(clips : Vec<Clip>, path:&String) -> (){
    fs::remove_file(&path)
    .expect("Something went wrong deleting the file");

    let mut clip_string = String::new();

    for clip in clips{
        clip_string = format!("{}\n{},{}", clip_string, clip.name, clip.code);
    }

    let write_result = fs::write(&path, clip_string);

    match write_result{
        Ok(v) => println!("Saved successfully"),
        Err(v) => println!("Error saving")
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
        Err(e) => panic!("couldnt read clipboard")
    }
}

fn get_help() -> (){
    let output = "cpd commands:
    \t- add/new: store a new Clip => ex: cpd add
    \t- remove/rm/delete + [index]: removes a stored Clip by index => ex: cpd remove 2
    \t- list/ls: lists all Clips => ex: cpd list
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

fn get_clip(vecman : &Vec<Clip>){
    let index_input = std::env::args().nth(2).expect("No further argument given, use 'cpd list' or 'cpd help'");
    let index : usize = index_input.parse().unwrap();
    if index-1 > vecman.len(){
        panic!("Invalid index!! cpd list for valid arguments");
    }
    println!("Copying code for {} to clipboard", vecman[index-1].name);
    write_to_clipboard(&vecman[index-1].code.to_string());
}

fn write_to_clipboard(cont : &String) -> (){
    let mut ctx : ClipboardContext = ClipboardProvider::new().unwrap();
    let res = ctx.set_contents(cont.to_string());
    
    match res {
        Ok(v) => println!("wrote {} to clipboard", cont),
        Err(e) => println!("{}",e)
    }
    let _ = ctx.get_contents();
}

fn remove_clip_cli(mut vecman : Vec<Clip>){
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
            list_clips(&vecman);
        },
        _  => {
            get_help();
        }
    }
}
