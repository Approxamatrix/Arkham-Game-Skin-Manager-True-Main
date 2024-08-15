use std::fs::*;
use std::io;
use std::io::BufWriter;
use std::io::Read;
use std::path;
use std::path::Path;
use std::path::PathBuf;
use std::process::exit;
use std::vec;
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

    let alreadysetup = read_maincfg(Path::new("maincfg.json")).startupcompleted;

    if alreadysetup == true{
       

    }

    else{

        startup();
    }
    

    let mut selectedprofile = String::new();


    let selectedprofile = profileselection();

    functionselection(selectedprofile);
    //I'll figure out the whole file structure stuff and reading the profile config file later.....
  // addmodfile();
  //change_mod_loadout();

 // startup();
    
    //change_mod_loadout();

  //  startup();

  //  remove_profile();
    //delete_mod();

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
#[derive(Debug,Clone)]
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
    startupcompleted : bool,
    profiles : Vec<ProfileInfo>

}


#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]

struct ProfileInfo{

    profilename : String,
    game : String,
    pathtoprofilecfg : String



}



#[derive(Serialize, Deserialize)]
#[derive(Debug,Clone)]
struct ProfileConfig{

    name : String,
    dlc_folder : String,
    //maybe put the game here via an enum ??
    mods : Vec<ModInfo>,





}

fn profileselection() -> String{

    let mut selectedprofile = String::new();

  

    let maincfgdata = read_maincfg(Path::new("maincfg.json"));

    let mut index = 0;

    for profile in &maincfgdata.profiles{

        println!("{}",index);
        println!("{}",profile.profilename);

        index = index + 1;

    }



    println!("Select a profile from the ones listed above !");

    io::stdin().read_line(&mut selectedprofile);
    
    selectedprofile = selectedprofile.trim().to_string();

    let mut index = 0;

    for profile in &maincfgdata.profiles{

        println!("{}",index);
        println!("{}",profile.profilename);

        index = index + 1;

        if selectedprofile == profile.profilename{

          //it should be fine ?? ( i have no idea)

          println!("successfully selected profile !");
        }

        else {
            
            println!("No profile exists with the profile you have entered");
            profileselection();
            return  "null".to_string();
        }

    }


    return selectedprofile;


}

fn functionselection(selectedprofile : String){

    let mut featureselection = String::new();
    
    println!("Please choose a feature ! : \n");

    println!("1. Add a new mod \n");
    println!("2. Delete a mod \n");
    println!("3. Change mod loadout \n");
    println!("4. Back to profiles menu \n");
    println!("6. Exit Program \n");

    io::stdin().read_line(&mut featureselection);

    featureselection = featureselection.trim().to_string();

    let featureselection : u8 = featureselection.parse().expect("failed to convert to unsigned 8 bit integer");

    match featureselection{

        1=> {import_mod(selectedprofile)},
        2=> {delete_mod(selectedprofile)},
        3=> {change_mod_loadout(selectedprofile)},
        4=> {profileselection();},
        _ => {functionselection(selectedprofile.clone())}

    }
    println!("stuff goes here... \n");

    //import_mod(selectedprofile);


   // readconfigfile(tempcfgpath);

//   change_mod_loadout()

  // readconfigfile();
    // a match statement will be used to select a menu item :3


}


fn startup(){




println!("Hello, would you like to set up a profile ?
\n1.Yes

\n2.No");

let mut profiledecision = String::new();

io::stdin().read_line(&mut profiledecision).expect("Failed to read input");


let mut profiledecision= profiledecision.trim();


let profiledecisionint = profiledecision.parse::<i8>().expect("failed to convert profile decision to int");

match profiledecisionint{

    1 => create_new_profile(),
    _ => println!("alright understood, the program will not create a profile.....\n\n"),

    

}



}


fn create_new_profile(){

    println!("Profile creation started ! \n\n");


    println!("Please enter a name for the profile");    

    let mut profilename: String = String::new();

    io::stdin().read_line(&mut profilename).expect("Failed to read input");


    let mut profilename= profilename.trim();

    println!("Please enter the name of the game you're making the profile for !");

    let mut gamename: String = String::new();

    io::stdin().read_line(&mut gamename).expect("Failed to read input");


    let mut gamename= gamename.trim();

    println!("Please wait.........\nCreating your profile.........");

    let mut newprofilestring = String::new();
    let mut newprofilestring = "profiles/".to_string() + profilename;
    create_dir(newprofilestring.clone()).expect("failed to create folder");


    let profiledata = create_profile_data(profilename.to_string(), gamename.to_string(), newprofilestring.to_string());

    let mut cfgdata = read_maincfg(Path::new("maincfg.json"));

    cfgdata.startupcompleted = true;

    cfgdata.profiles.push(profiledata);

    println!("{:#?}",cfgdata);

    let mut file : File = File::create("maincfg.json").expect("Failed to read cfg file !");


    to_writer_pretty(&file, &cfgdata);

    println!("Finished completing profile !");
    







}

fn read_maincfg(cfgpath : &Path)-> MainConfig{

    let cfgfile = File::open(cfgpath).expect("failed to open cfg file path !");
    
    let bufferedreader = BufReader::new(cfgfile);

    let cfgdata : MainConfig = from_reader(bufferedreader).expect("failed to fetch data from config file !");

    //.expect("failed to parse data from cfg JSON file !");
    

    println!("{:#?} \n",cfgdata);


    return cfgdata;




}
fn create_profile_data(profilename: String,game: String,pathtoprofilecfg: String) ->ProfileInfo {

    let profiledata = ProfileInfo{
        
        profilename,
        game,
        pathtoprofilecfg,
    

    };



    return profiledata;

}

fn remove_profile(){

    let mut profilename: String = String::new();

    io::stdin().read_line(&mut profilename);

    let mut profilename = profilename.trim();

    for dir in read_dir("profiles").expect("idklmao"){
        
        let dir = dir.expect("e");
        let path = dir.path();
        


        if path.is_dir(){

            if (path.file_name().expect("idk").to_string_lossy().to_string() == profilename){


                remove_dir_all(path);
                println!("successfully removed profile directory.")

            }


        }


    }

    delete_profiledata(profilename.to_string());

}

fn delete_profiledata(profilename: String){
   
    let mut profiledata: MainConfig = read_maincfg(Path::new("maincfg.json"));

    let mut index = 0;

    for profile in profiledata.profiles.clone(){

        
        if profile.profilename == profilename{
            profiledata.profiles.remove(index);
            println!("{}",profilename);
            
        }

        index = index + 1;

    }

    
    
    let mut file : File = File::create("maincfg.json").expect("Failed to read cfg file !");

    
    to_writer_pretty(&mut file, &profiledata).expect("Failed to write to file !");



    //look through profiles
    // see if profile name is the same as the profile name provided,
    // if it is, then delete it


}

fn import_mod(selectedprofile : String) {

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

    let destpath:String = modpathvalue(selectedprofile.clone());
    
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

    let cfgfile = readconfigfile(Path::new(&("profiles\\".to_owned() + &selectedprofile.clone() +"\\config\\profileconfig.json")));

    add_mod_to_cfg(cfgfile,addedmodstruct,selectedprofile);

   
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


fn modpathvalue(selectedprofile : String)  -> String {

    let config = File::open("profiles\\".to_owned() + &selectedprofile + "\\config\\profileconfig.json").expect("failed to read file !");
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

   
fn add_mod_to_cfg(mut cfgdata : ProfileConfig,jsonformatteddata : ModInfo, selectedprofile : String)
{

    //println!("function has received the following json file :");
    //println!("\n\n\n\n{:#?}",cfgdata);

    
    println!("number of mods : {}",cfgdata.mods.len());


    let modcount = cfgdata.mods.len();

    cfgdata.mods.push(jsonformatteddata);

    println!("{:#?}",cfgdata);

    let mut file : File = File::create("profiles\\".to_owned() + &selectedprofile+ "\\config\\profileconfig.json").expect("Failed to read cfg file !");

    //let cfgdata = to_string(&cfgdata).expect("failed to convert to string")
    to_writer_pretty(&mut file, &cfgdata).expect("Failed to write to file !");
}

fn delete_mod(selectedprofile : String){

    let mut cfgfile = readconfigfile(Path::new(&( "profiles\\".to_owned() + &selectedprofile.clone() + "\\config\\profileconfig.json")));

    let mut index = 0;
    for mods in &cfgfile.mods{

        let index = index + 1;
        println!("{:#?}",index);
        println!("{:#?}",mods.modname);



    }

    let index = 0;
    let mut modselection = String::new();
    io::stdin().read_line(&mut modselection).expect("failed to read line !");

    let mut modselection = modselection.trim();

    let mut modselection:usize = modselection.parse().expect("failed to parse selection !");

    let mut modselection = modselection - 1;


    std::fs::remove_dir_all( &mut cfgfile.mods[modselection].modfolderpath).expect("failed to delete directory");

    
    std::fs::remove_dir_all( &mut cfgfile.mods[modselection].ogmodlocation).expect("failed to delete directory");


    let alteredmodlist = remove_mod_from_cfg( cfgfile.clone(),index.try_into().unwrap());

    
    let mut file : File = File::create(&(selectedprofile.clone() + "\\config\\testconfig.json")).expect("Failed to read cfg file !");

    cfgfile.mods = alteredmodlist.mods;
    to_writer_pretty(&mut file, &cfgfile).expect("Failed to write to file !");


    println!("Deleted mod !");

}


fn remove_mod_from_cfg(mut cfgdata : ProfileConfig, vecindex : i32) -> ProfileConfig{

    println!("{}",vecindex);
    cfgdata.mods.remove(vecindex.try_into().unwrap());
    return cfgdata

   


}

fn change_mod_loadout(selectedprofile : String)
{
    
    let mut jsonfile = readconfigfile(Path::new(&(selectedprofile.clone() + ("\\profileconfig.json"))) );
    let mut modindex = 0;

    let mut file : File = File::create(selectedprofile + ("\\profileconfig.json")).expect("Failed to read cfg file !");


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