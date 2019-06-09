use reqwest;
use reqwest::StatusCode;
use std::fs::File;
use std::fs::create_dir_all;
use std::io;
use std::path::Path;

pub fn download_file(url: String, path: &str) {
    create_dir_all(path);

    let url_parts: Vec<&str> = url.split('/').collect();
    let output = Path::new(path).join(url_parts.last().unwrap());

    match reqwest::get(url.as_str()) {
        Ok(mut resp) => {
            match resp.status() {
                StatusCode::OK => (),
                _ => {
                    println!("Could not download this file: {}", url);
                    return;
                },
            }
            let mut file = match File::create(&output) {
                Ok(f) => f,
                Err(err) => {
                    println!("Error occurred while creating file: {} | Error: {}", output.display(), err);
                    return;
                }
            };
            match io::copy(&mut resp, &mut file) {
                Ok(_) => println!("File {} has been downloaded", output.display()),
                Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
            }
        },

        Err(err) => println!("Could not download this file: {} | Error: {}", url, err),
    };
}
