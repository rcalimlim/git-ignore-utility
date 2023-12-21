use clap::{arg, Command};
use std::{fs::File, io::Write};

fn main() {
    const API_URL: &str = "https://www.toptal.com/developers/gitignore/api";

    let cmd = Command::new("gi")
        .arg(arg!([keywords] "Comma separated keywords to generate a .gitignore for."))
        .get_matches();

    if let Some(keywords) = cmd.get_one::<String>("keywords") {
        let result = reqwest::blocking::get(format!("{}/{}", API_URL, keywords));

        if let Ok(res) = result {
            if let Ok(mut file) = File::create(".gitignore") {
                if let Ok(bytes) = &res.bytes() {
                    let write_res = file.write(&bytes);
                    match write_res {
                        Ok(_) => println!("Successfully create .gitignore for {}", keywords),
                        Err(err) => println!("Error: {}", err),
                    }
                } else {
                    println!("Could not get .gitignore for arg `{}`", keywords);
                }
            }
        } else {
            println!("Could not get .gitignore for arg `{}`", keywords);
        }
    }
}
