use clap::{arg, ArgAction, Command};
use std::{fs::File, io::Write};

fn main() {
    const API_URL: &str = "https://www.toptal.com/developers/gitignore/api";

    let cmd = Command::new("git-ignore-utility")
        .about("Generate .gitignore files based on a search term. Example: `gi rust` would produce a .gitignore file with `/target` ignored.")
        .author("Ross F. Calimlim, ross.calimlim@gmail.com")
        .bin_name("gi")
        .arg(arg!([keywords] "Comma separated keywords to generate a .gitignore for.").required(true))
        .arg(arg!(-c --create ... "Create a file rather than print to stdout [WIP]").action(ArgAction::SetTrue))
        .get_matches();

    if let Some(keywords) = cmd.get_one::<String>("keywords") {
        let result = reqwest::blocking::get(format!("{}/{}", API_URL, keywords));

        if let Ok(res) = result {
            match cmd.get_one::<bool>("create") {
                Some(&should_create_file) => {
                    if should_create_file {
                        // create a file
                        if let Ok(mut file) = File::create(".gitignore") {
                            if let Ok(bytes) = &res.bytes() {
                                let write_res = file.write(&bytes);
                                match write_res {
                                    Ok(_) => {
                                        println!("Successfully create .gitignore for {}", keywords)
                                    }
                                    Err(err) => println!("Error: {}", err),
                                }
                            } else {
                                println!("Could not get .gitignore for arg `{}`", keywords);
                            }
                        }
                    } else {
                        // else print to stdout
                        println!("{}", res.text().unwrap());
                    }
                }
                None => {
                    println!(".gitignore for {}\n{}", keywords, res.text().unwrap());
                }
            }
        } else {
            println!("Could not get .gitignore for arg `{}`", keywords);
        }
    }
}
