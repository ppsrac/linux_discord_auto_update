use std::fs;
use std::path::PathBuf;
use dirs;
use rfd::MessageDialog;
use std::process::Command;

fn detect_home_dir()->PathBuf{
    match dirs::home_dir(){
        Some(path) => path,
        _ => {
            error_message("Cannot detect your home directory");
            std::process::exit(1);
        }
    }
}

#[allow(dead_code)]
fn read_files(path: &PathBuf)->Vec<PathBuf>{
    let mut files: Vec<PathBuf> = Vec::new();
    for entry in fs::read_dir(path).unwrap_or_else(|_|{
        error_message("Cannot read your files in your directory");
        std::process::exit(1);
    }){
        files.push(entry.unwrap_or_else(|_|{
            error_message("Cannot read your file");
            std::process::exit(1);
        }).path());
    }
    files
}

fn error_message(msg: &str){
    MessageDialog::new()
        .set_title("Error")
        .set_description(msg)
        .set_level(rfd::MessageLevel::Error)
        .show();
}

fn download_and_install_discord(password: &String){
    let command = ["-O", "discord.deb", "https://discordapp.com/api/download?platform=linux&format=deb"];
    let output = Command::new("wget")
        .args(command)
        .output();
    match output{
        Err(_) =>{
            error_message("Failed to download new discord");
            std::process::exit(1);
        },
        Ok(output) if !output.status.success() =>{
            let error = std::str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            println!("error: {error}");
            error_message(&format!("Failed to download Discord: {}", error));
            std::process::exit(1);
        },
        Ok(output)=>{
            println!("Download Succeeded");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };

    let new_command = format!("echo {} | sudo -S dpkg -i discord.deb", password);
    let output = Command::new("sh")
        .arg("-c")
        .arg(new_command)
        .output();
    match output{
        Err(_) =>{
            error_message("Failed to download new discord");
            std::process::exit(1);
        },
        Ok(output) if !output.status.success() =>{
            let error = std::str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            println!("error: {error}");
            error_message(&format!("Failed to install Discord: {}", error));
            std::process::exit(1);
        },
        Ok(output)=>{
            println!("Install succeeded");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };

    let output = Command::new("rm")
        .args(["-rf", "discord.deb"])
        .output();
    match output{
        Err(_) =>{
            error_message("Failed to download new discord");
            std::process::exit(1);
        },
        Ok(output) if !output.status.success() =>{
            let error = std::str::from_utf8(&output.stderr).unwrap_or("Unknown error");
            println!("error: {error}");
            error_message(&format!("Failed to delete Discord: {}", error));
            std::process::exit(1);
        },
        Ok(output)=>{
            println!("Delete succeeded");
            String::from_utf8_lossy(&output.stdout).to_string()
        }
    };
}

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let password = &args[1];
    download_and_install_discord(password);
}