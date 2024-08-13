use std::fs::*;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use fs_extra::*;
use serde::*;
use serde_json::from_reader;
use serde_json::from_str;
use serde::{Deserialize, Serialize};
use serde_json::to_string;
use serde_json::to_writer;
use serde_json::to_writer_pretty;
use serde_json::Value;
use std::borrow::Cow;
use serde_json::json;
use std::io::BufReader;
use std::cmp;
use std::error::Error;

fn main() {

    //I'll figure out the whole file structure stuff and reading the profile config file later.....
  // addmodfile();
  //change_mod_loadout();

 // startup();
    change_mod_loadout();

}
/*
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ModInfo<'a>{

    modname : &'a str,
    #[serde(borrow)]
    modfolderpath :  Cow<'a, str>,
    active : bool,

}


#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ProfileConfig<'a>{

    name : &'a str,
    #[serde(borrow)]
    dlc_folder : Cow<'a, str>,
    //maybe put the game here via an enum ??
    mods : Vec<ModInfo <'a>>,


*/
    
#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ModInfo{

    modname : String,
    ogmodlocation : String,
    rootmodfolder: String,
    modfolderpath : String,
    active : bool,

}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct MainConfig{
    mod_backup_folder : String,
  //  profiles : Vec<String>

}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
struct ProfileConfig{

    name : String,
    dlc_folder : String,
    //maybe put the game here via an enum ??
    mods : Vec<ModInfo>,





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
    //addmodfile();


   // let tempcfgpath : &Path = Path::new("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json");
   // readconfigfile(tempcfgpath);

//   change_mod_loadout()

  // readconfigfile();
    // a match statement will be used to select a menu item :3


}


fn startup(){

let maincfg = File::open("maincfg.json").expect("the main config file doesn't exist !!!");



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

    scandlcfolder(destpath.to_string());

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

    

    //Time to make a function that scans the dlc folder and returns an array or vector containing the paths for each one.
    // then the vector is iterated through in a for loop.
    // the user has to input a number corresponding to the array index.
    // upon sending the input, a reference to that input index is taken and the string in that index is sent to the copy function





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


    //Time to mess with structs

    let addedmodstruct = ModInfo{

        modname : modfilename.expect("idk").to_string(),
        ogmodlocation : modfilename.expect("idk").to_string(), //leave this until later.... too eepy
        rootmodfolder : folder_array[folderselectionint].to_string_lossy().to_string(), //LMFAOOOOOO what on earth is this monstrosity of a string conversion !?!?! XD
        modfolderpath : truefinalmodfilepath.into(),
        active : true

    };

    //let jsonformatteddata = to_string(&addedmodstruct).expect("failed to convert json to string !");


   // println!("{:#?} \n",jsonformatteddata);
    
  // println!("Name of file : {}",modfileconverted.file_name());
    

    // first, prompt for the path of the file
    // second, prompt for the path you want to copy the file into (later on, when the config and stuff are setup, 
    // it will look into the arkham game's dlc folder 
    // and ask you which one you want to copy the file into)

    // next, pass the paths into std::fs::copy and let the magic happen !

    let cfgfile = readconfigfile(Path::new("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json"));

    add_mod_to_cfg(cfgfile,addedmodstruct);

   
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



fn readconfigfile(cfgpath : &Path) -> ProfileConfig {

// this will use the serde_json crate to read the file and print the info.

// use for loops for this , okie dokie ?
   // println!("Name : {}\n", json0["name"]);


    let cfgfile = File::open(cfgpath).expect("failed to open cfg file path !");
    
    let bufferedreader = BufReader::new(cfgfile);

    let cfgdata : ProfileConfig = from_reader(bufferedreader).expect("failed to fetch data from config file !");

    //.expect("failed to parse data from cfg JSON file !");

    println!("{:#?} \n",cfgdata);

    println!("The number of mods present is....... {:#?} !\n",cfgdata.mods.len());

    return cfgdata;

}



    //let config : ProfileConfig = serde_json::from_reader().expect("Oh noes ! Failed to parse JSON !");
   // println!("{}",config.name);
    //for mods in 1 ..11 {

        
    /*  println!(
        "Mod name : {} \nMod Path : {} \nPath In Game Files : {} \nActive ? : {}", 
       
    json0["mods"][mods]["modname"],
        json0["mods"][mods]["modfolderpath"],
        json0["mods"][mods]["pathingame"],
        json0["mods"][mods]["active"]
        
    ) */
    
        
   // }

   
fn add_mod_to_cfg(mut cfgdata : ProfileConfig,jsonformatteddata : ModInfo)
{

    //println!("function has received the following json file :");
    //println!("\n\n\n\n{:#?}",cfgdata);

    
    println!("number of mods : {}",cfgdata.mods.len());


    let modcount = cfgdata.mods.len();

    cfgdata.mods.push(jsonformatteddata);

    println!("{:#?}",cfgdata);

    let mut file : File = File::create("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json").expect("Failed to read cfg file !");

    //let cfgdata = to_string(&cfgdata).expect("failed to convert to string")
    to_writer_pretty(&mut file, &cfgdata).expect("Failed to write to file !");
}

fn change_mod_loadout()
{
    
    let mut jsonfile = readconfigfile(Path::new("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json"));
    let mut modindex = 0;

    let mut file : File = File::create("C:\\Users\\Anish\\Desktop\\testingcopy\\config\\testconfig.json").expect("Failed to read cfg file !");


    for mods in &jsonfile.mods
    { 
        modindex = modindex+1;
        println!("{} . Mod : {:#?} \nEnabled ? : {}\n\n",&modindex,mods.modname,mods.active);
    }

    println!("enter the index of the mod you want to toggle");

    let mut modselection : String = String::new();

    io::stdin().read_line(&mut modselection).expect("failed to read line !");

    let mut modselection = modselection.trim();

    let mut modselection:usize = modselection.parse().expect("failed to parse selection !");

    let mut modselection = modselection - 1;
    //println!("{:#?}",jsonfile.mods[modselection]);

    

    if (jsonfile.mods[modselection].active == true){

        jsonfile.mods[modselection].active = false;
        std::fs::remove_dir_all( &jsonfile.mods[modselection].modfolderpath).expect("failed to delete directory");
    }

    else{

        let copyoptionstuff = dir::CopyOptions::new().overwrite(true);
        let mut copypathvec = Vec::new();
        copypathvec.push(jsonfile.mods[modselection].ogmodlocation.clone());
        

        jsonfile.mods[modselection].active = true;
        fs_extra::copy_items(&copypathvec, &jsonfile.mods[modselection].rootmodfolder,&copyoptionstuff).expect("failed to copy mod !");
    

    }

  //  println!("{:#?}",jsonfile.mods[modselection]);

    to_writer_pretty(&mut file, &jsonfile).expect("Failed to write to file !");

}