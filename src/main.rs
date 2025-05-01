use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;
use rand::Rng;
use dirs;

fn main() {
    let args: Vec <String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("error! usage: {} <password_size>", args[0]);
        return;
    }

    let pass_size = match args[1].parse::<usize>() {
        Ok(size) => size,
        Err(_) => {
            eprintln!("error! password size must be an integer.");
            return;
        }
    };

    println!("generating password with {} digits...", pass_size);
    let password = generate_password(pass_size);
    println!("generated password: {}", password);

    let file_path = get_file_path();

    save_to_file(&file_path, &password).expect("failed to write file.");
    
    println!("writing password...");
    println!("password saved at: {}", file_path.display());
}

fn generate_password(length: usize) -> String {
    let lowercase: Vec<char> = ('a'..='z').collect();
    let uppercase: Vec<char> = ('A'..='Z').collect();
    let numbers: Vec<char> = ('0'..='9').collect();
    let symbols: Vec<char> = "!@#$%&*+-=/?_".chars().collect();
    
    let all_characters = [lowercase, uppercase, numbers, symbols].concat();

    let mut rng = rand::rng();
    let password: String = (0..length)
    .map(|_| {
            let idx = rng.random_range(0..all_characters.len());
            all_characters[idx]
        })
        .collect();

    password
}

fn get_file_path() -> PathBuf {
    println!("where do you wish to save your password? (press enter for ~/.passgen/): ");
    let mut directory = String::new();
    io::stdin().read_line(&mut directory).expect("failed to read input.");
    let directory = directory.trim();

    let path = if directory.is_empty() {
        dirs::home_dir()
            .expect("could not find home directory...")
            .join(".passgen")
    } else {
        PathBuf::from(directory)
    };

    if !path.exists() {
        fs::create_dir_all(&path).expect("failed to create path.");
    }

    println!("what should be the file name? (press enter for password.txt): ");
    let mut filename = String::new();
    io::stdin().read_line(&mut filename).expect("failed to read input.");
    let filename = filename.trim();

    let filename = if filename.is_empty() {
        "password.txt".to_string()
    } else {
        if !filename.ends_with(".txt") {
            format!("{}.txt", filename)
        } else {
            filename.to_string()
        }
    };

    path.join(filename)
}

fn save_to_file(path: &PathBuf, content: &str) -> io::Result<()> {
    let mut file = fs::File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

