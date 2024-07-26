use std::fs::*;
use std::io;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use fs_extra::*;
use serde::*;
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;
use serde_json::json;


fn main() {

    //I'll figure out the whole file structure stuff and reading the profile config file later.....
   functionselection();
}

#[derive(Serialize, Deserialize)]
struct ModInfo<'a>{

    modname : String,
    #[serde(borrow)]
    modfolderpath :  Cow<'a, str>,
    #[serde(borrow)]
    pathingame :  Cow<'a, str>,
    active : bool,

}

#[derive(Serialize, Deserialize)]
struct ProfileConfig<'a>{

    name : String,
    #[serde(borrow)]
    dlc_folder : Cow<'a, str>,
    //maybe put the game here via an enum ??
    mods : Vec<ModInfo <'a>>,





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
    addmodfile();
  // readconfigfile();
    // a match statement will be used to select a menu item :3


}

fn addmodfile() {

    println!("Please drag the mod file into the terminal, okie ?\n");

    let mut modfilepath = String::new();

    io::stdin().read_line(&mut modfilepath).expect("Hey dipshit !! what the fuck did you do ???? \n");

    let modfilepath = modfilepath.trim();

    let mut sourcepath = Vec::new();
    sourcepath.push(modfilepath);

    //println!("Now enter the path where you want to copy the file into : \n");
   // let mut destpath = String::new();


//TODO : Instead of asking the user for the destination right away, check the mod folder and then scan the folders in that folder.

// soo i guess i gotta make a function that returns the dlc path ??? also, write some code that scans the dlc folder for subfolders and displays them 
//and lets you choose




    //io::stdin().read_line(&mut destpath).expect("How did you break this ???? LMAO \n");

    let destpath:String = modpathvalue();

    let destpath: &str = destpath.trim();
       
    println!("{destpath \n}");

    scandlcfolder(destpath.to_string());



    println!("Okay that's all the info I needed :D\n");

   // let folder_array = scandlcfolder(destpath.to_string());


    //Time to make a function that scans the dlc folder and returns an array or vector containing the paths for each one.
    // then the vector is iterated through in a for loop.
    // the user has to input a number corresponding to the array index.
    // upon sending the input, a reference to that input index is taken and the string in that index is sent to the copy function




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
   

  //  fs_extra::copy_items(&sourcepath, destpath, &copyoptionstuff);
    
    // first, prompt for the path of the file
    // second, prompt for the path you want to copy the file into (later on, when the config and stuff are setup, 
    // it will look into the arkham game's dlc folder 
    // and ask you which one you want to copy the file into)

    // next, pass the paths into std::fs::copy and let the magic happen !


   
}

fn scandlcfolder(dlcfolderpath : String) -> Vec<PathBuf>{

    let mut subfolders: Vec<PathBuf> = Vec::new();
    for entry in read_dir(dlcfolderpath.to_string()).expect("What ? Help me !") {
        let entry = entry.expect("uhhh failed to parse the entry in the JSON file ??? idk");
        let path = entry.path();

        if path.is_dir() {
            //print!("{:?} \n", entry.path());
            subfolders.push(path);
            // now have this path be pushed into a vector and then return it.
            
        }
        //
        
        
    }
    println!("{:?}",subfolders);
    return subfolders;




}


fn modpathvalue() -> String{

    let json0= 
        r#"{


            "name" : "idk",
            "dlc_folder" : "C:\\Users\\Anish\\Desktop\\testingcopy\\dlc",
            "mods" :
            [
                {
                    "modname" : "Man",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
        
                {
                    "modname" : "idk man",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
                {
                    "modname" : "idk man2",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                }
               
            ]
                       
     
        }"#;


        let parsedconfig : ProfileConfig = serde_json::from_str(&json0).expect("Oh noes ! Failed to parse JSON !");
        return parsedconfig.dlc_folder.to_string();







}

fn readconfigfile() {

// this will use the serde_json crate to read the file and print the info.
    let json0= 
        r#"{


            "name" : "idk",
            "dlc_folder" : "e",
            "mods" :
            [
                {
                    "modname" : "Man",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
        
                {
                    "modname" : "idk man",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                },
        
                {
                    "modname" : "idk man2",
                    "modfolderpath" : "idk_lmao",
                    "pathingame" : "idk",
                    "active" : true
                }
               
            ]
                       
     
        }"#;

// use for loops for this , okie dokie ?
   // println!("Name : {}\n", json0["name"]);

    let config : ProfileConfig = serde_json::from_str(&json0).expect("Oh noes ! Failed to parse JSON !");
    println!("{}",config.name);
    //for mods in 1 ..11 {

        
    /*  println!(
        "Mod name : {} \nMod Path : {} \nPath In Game Files : {} \nActive ? : {}", 
       
    json0["mods"][mods]["modname"],
        json0["mods"][mods]["modfolderpath"],
        json0["mods"][mods]["pathingame"],
        json0["mods"][mods]["active"]
        
    ) */
    
        
   // }

   
}