use std::fs::*;
use std::io;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use fs_extra::*;
use serde::*;
use serde_json::from_reader;
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::borrow::Cow;
use serde_json::json;
use std::io::BufReader;
use std::cmp;

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
    let modfileconverted: PathBuf = modfilepath.clone().parse::<PathBuf>().expect("Failed to get the path for the mod file or smth idk");
    println!("{:?}",modfilepath);
   // let modfilepath = modfilepath.as_str();

    let mut sourcepath = Vec::new();
    sourcepath.push(modfilepath);



    //println!("Now enter the path where you want to copy the file into : \n");
   // let mut destpath = String::new();


//TODO : Instead of asking the user for the destination right away, check the mod folder and then scan the folders in that folder.

// soo i guess i gotta make a function that returns the dlc path ??? also, write some code that scans the dlc folder for subfolders and displays them 
//and lets you choose

    //io::stdin().read_line(&mut destpath).expect("How did you break this ???? LMAO \n");

    let destpath:String = modpathvalue();
    
    // let destpath: &str = destpath.trim();
       
    println!("{destpath \n}");

    //scandlcfolder(destpath.to_string());

    println!("Okay that's all the info I needed :D\n");

    let folder_array = scandlcfolder(destpath);

    let mut counter :u32 = 0;
    println!("Please enter the number corresponding to the folder that you want to add the mod to ! \n \n \n");
    for folders in &folder_array{

        println!("{} ---- {:?}",counter,folders);
        
        //println!("{:?}",folders);

        counter = counter + 1;

        //println!("{}", folder_array[folders]);



    };

    let mut folderselectionstring = String::new();

    io::stdin().read_line(&mut folderselectionstring).expect("Not sure how you broke this but you did. Somehow you broke the selection thing for choosing which mod folder");
    
    let folderselectionstring = folderselectionstring.trim();

    let folderselectionint : usize = folderselectionstring.parse().expect("failed to parse string");

    

    println!("{:?}",folder_array[folderselectionint]);

    

   // println!("{:?}",folder_array);
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
   

    fs_extra::copy_items(&sourcepath, &folder_array[folderselectionint], &copyoptionstuff);
    

    println!("Folder selected : {:?} \n",&folder_array[folderselectionint]);

    let modfilename ;

    let decoystring = "idk";
    
    match modfileconverted.file_name(){
    
    Some(idk) => modfilename = idk.to_str() ,    
    None => { modfilename = Some(decoystring);
        println!("something went wrong with getting the file name")},


    };

    let truefinalmodfilepath : String = (folder_array[folderselectionint].to_string_lossy() + "\\").to_string() + modfilename.expect("failed to get the mod's directory's name !") ;

    println!("{}",truefinalmodfilepath);
    
  // println!("Name of file : {}",modfileconverted.file_name());
    

    // first, prompt for the path of the file
    // second, prompt for the path you want to copy the file into (later on, when the config and stuff are setup, 
    // it will look into the arkham game's dlc folder 
    // and ask you which one you want to copy the file into)

    // next, pass the paths into std::fs::copy and let the magic happen !


   
}

fn scandlcfolder(dlcfolderpath : String) -> Vec<PathBuf>{

    let mut subfolders: Vec<PathBuf> = Vec::new();
   // let dlcfolderpath1 = "C:\\Program Files\\Epic Games\\BatmanArkhamKnight\\DLC";

    println!("{:?}",dlcfolderpath);
    //println!("{:?}",dlcfolderpath1);
    for entry in read_dir(dlcfolderpath).expect("What ? Help me !") {
        let entry = entry.expect(" e ");
        let path = entry.path();

        if path.is_dir() {
            //print!("{:?} \n", entry.path());
            subfolders.push(path);
            // now have this path be pushed into a vector and then return it.
            
        }
        //
        
        
    }
     
  //  println!("{:?}",subfolders);
    println!("\nThere are {} dlc folders \n",subfolders.len());
    
    return subfolders;




}


fn modpathvalue()  -> String {

    let config = File::open("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json").expect("failed to read file !");
    let configreader = BufReader::new(config);
    let configread : Value = from_reader(configreader).expect("could not read json file ! ...i think ??? idk how this one works too well so idk...");
    // let parsedconfig : ProfileConfig = serde_json::from_str(&json0).expect("Oh noes ! Failed to parse JSON !");
    
    let dlcfolderpath = configread.get("dlc_folder").expect("couldn't read the DLC folder string from the config file !");
    println!("{:?}",dlcfolderpath.to_string());
    let dlcpathbutstr = dlcfolderpath.as_str().expect("Hmmm... I'm  pretty sure a string should be used here.....");
    println!("{:?}",dlcpathbutstr);
    //LMAO why does this work. this seems so jank to me lol. but i guess that's my inexperience showing :3c


    return dlcpathbutstr.to_string();
  






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