use std::fs::*;
use std::io;
use std::path::Path;
use fs_extra::*;
use serde::*;
use serde_json::json;
fn main() {

    //I'll figure out the whole file structure stuff and reading the profile config file later.....
   functionselection();
}


struct modinfo{

    modname : String,
    modfolderpath : String,
    pathingame : String,
    active : bool,

}

struct profileconfig{

    profilename : String,
    //maybe put the game here via an enum ??
    mods : [modinfo],





}

fn profileselection(){


    println!("Hello, world!");

    println!("Create a new profile or select one ?");

    println!("placeholder1");

    println!("placeholder2");

    println!("placeholder3");

    println!("Create a new profile ! :D");



}

fn functionselection(){

    println!("stuff goes here... \n");

    println!("1. Add a new mod \n");
    println!("2. Delete a mod \n");
    println!("3. Change mod loadout \n");
    println!("4. Save mod loadout \n");
    println!("5. Back to profiles menu \n");
    println!("6. Exit Program \n");
   /// obtainmodfile();
   readconfigfile()
    // a match statement will be used to select a menu item :3


}

fn obtainmodfile() {

    println!("Please drag the mod file into the terminal, okie ?\n");

    let mut modfilepath = String::new();

    io::stdin().read_line(&mut modfilepath).expect("Hey dipshit !! what the fuck did you do ???? \n");

    let modfilepath = modfilepath.trim();

    let mut sourcepath = Vec::new();
    sourcepath.push(modfilepath);

    println!("Now enter the path where you want to copy the file into : \n");
    let mut destpath = String::new();

    io::stdin().read_line(&mut destpath).expect("How did you break this ???? LMAO \n");

    let destpath: &str = destpath.trim();

    println!("Okay that's all the info I needed :D\n");
/*
    let renaming = rename(modfilepath.clone(), destpath.clone());

    let renamingresult: Result<(), std::io::Error> = match renaming{
        Ok(_) => {
            println!("File copied successfully !");
            Ok(())

        }
        Err(e) => {
            
            println!("Failed to copy mod ! D: ");

            Err(e)
        

        }


    };
 */

    
//Todo okay, since i'm using the fs_extra crate now, i can use it to copy a directory to a location. so don't write my own code for that
// just use the crate for that.


    let copyoptionstuff = dir::CopyOptions::new();
   // println!("{modfilepath}");
    println!("{destpath}");

    fs_extra::copy_items(&sourcepath, destpath, &copyoptionstuff);
    
    // first, prompt for the path of the file
    // second, prompt for the path you want to copy the file into (later on, when the config and stuff are setup, 
    // it will look into the arkham game's dlc folder 
    // and ask you which one you want to copy the file into)

    // next, pass the paths into std::fs::copy and let the magic happen !


   
}


fn readconfigfile(){

// this will use the serde_json crate to read the file and print the info.
    let json = json!(
        {


            "name" : "idk",
        
            "mods" :
            [
                {
                    "modname" : "Man",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
        
                {
                    "modname" : "Lego Batman",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
                {
                    "modname" : "The Batman",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                }
               
            ]
                       
     
        }


    );

// use for loops for this , okie dokie ?
    println!(
    "Name: {} \nMod name : {} \nMod Path : {} \nPath In Game Files : {} \nActive ? : {}", 
    json["name"], 
    json["mods"][0]["modname"],
    json["mods"][0]["modfolderpath"],
    json["mods"][0]["pathingame"],
    json["mods"][0]["active"]
)


}