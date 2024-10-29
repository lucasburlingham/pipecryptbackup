// Include the crypto module from src/crypto.rs
use crate::crypto;

use colored::Colorize;
use whoami;
use std::fs;
use std::io;
use ini::Ini;



fn write_initial_info_to_ini(
    user_home_path: String,
    current_user: String,
    key: &[u8],
    nonce: &[u8],
) {
    // Create user_home_path/.config/pipecryptbackup directory
    let config_dir_path = format!("{}/.config/pipecryptbackup", user_home_path);
    let crypto_dir = format!("{}/.config/pipecryptbackup/crypto", user_home_path);

    // Create the directories
    fs::create_dir_all(&crypto_dir).expect("Unable to create the crypto directory");
    fs::create_dir_all(&config_dir_path).expect("Unable to create the config directory");

    // Create the key and nonce file path
    let key_path = format!("{}/.config/pipecryptbackup/crypto/key_nonce.bin", user_home_path);

	// convert into a Path object
	let key_path_obj = std::path::Path::new(&key_path);

    // Store the key and nonce in the key_nonce.bin file
    crypto::store_key_nonce(key, nonce, key_path_obj).expect("Failed to store key and nonce");

    // Create the ini file path
    let ini_path = format!("{}/.config/pipecryptbackup/config.ini", user_home_path);

    // Create the ini object with the information gathered
    let mut conf = Ini::new();
    conf.with_section(Some("User"))
        .set("username", current_user.clone())
        .set("home_path", user_home_path.clone());

    // Write the ini object to the ini file
    conf.write_to_file(ini_path.clone()).expect("Unable to write to the ini file");

    println!("Initial information written to the ini file for user {}: {}", current_user, ini_path);
    println!("Key and nonce written to the key_nonce.bin file: {}", key_path);
}


pub fn init() {
    // Get the contents of the /home directory and look for the folder named after the current username
    let paths: Vec<_> = fs::read_dir("/home/").unwrap().collect::<Result<Vec<_>, _>>().unwrap(); // Collect into a Vec<Result<_, _>> and unwrap
    let current_user = whoami::username();
    let mut user_home_path = String::new();

    // Print the welcome message
    println!("{}", "Starting pipecryptbackup. (C) 2024 Lucas Burlingham.".blue());

    // Loop until the user_home_path is set
    loop {
        let mut found = false;

        for entry in &paths {
            let original_path = entry.path().display().to_string(); // Convert the path to a string
            let path = original_path.split('/').last().unwrap(); // Get the last part of the path

            // Print the paths of all the folders in the /home/ directory
            println!("Found folder {}", original_path);

            // Check if the folder is named after the current user
            if path == current_user {
                println!(
                    "Found home directory named after user {} in {}",
                    current_user.bold(),
                    original_path
                );
                user_home_path = original_path; // Set the user_home_path
                found = true;
                break; // Exit the for loop
            }
        }

        // If user_home_path was not found, prompt for input
        if !found {
            println!(
                "No folder named after current user found. Please enter the full path for the home directory for the user {}",
                current_user.bold()
            );

            // Read user input for home directory path
            let mut user_home_path_input = String::new();
            io::stdin().read_line(&mut user_home_path_input).unwrap();
            user_home_path = user_home_path_input.trim().to_string(); // Set the user_home_path
            break; // Exit the loop
        } else {
            break; // Exit the loop if home directory was found
        }
    }

    // Print the current logged-in user and their home path
    println!("Current logged-in user: {}", current_user.bold());
    println!("User home path: {}", user_home_path);

    // Generate the key and nonce to be used for encryption
    let (key, nonce) = crypto::generate_key_nonce();

    // Write the initial information to the ini file, including:
    // - The current user's username
    // - The current user's home path
    // - The key and nonce used for encryption
    write_initial_info_to_ini(user_home_path.clone(), current_user, &key, &nonce);

    // Get list of all files in the home directory
    let files = fs::read_dir(&user_home_path).unwrap();

    // Print the files in the home directory
    for file in files {
        let file = file.unwrap();
        let path = file.path();
        println!("Found file: {}", path.display());
    }
}