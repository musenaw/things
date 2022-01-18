extern crate image_base64;
use std::fs;
use std::fs::File;
use std::io::{prelude::*, ErrorKind};

fn main() -> std::io::Result<()> {
    for file in fs::read_dir("/home/sangulo/Imágenes/").unwrap() {
        let is_file = file.as_ref().unwrap().path().is_file();
        if is_file {
            let filename = file.unwrap().file_name().into_string();
            match filename {
                Ok(res) => {
                    if res.ends_with(".png") {
                        let image_path = "/home/sangulo/Imágenes/".to_string() + &res.to_string();
                        let image_path_output =
                            "/home/sangulo/Imágenes/b65/".to_string() + &res.to_string() + "_b64";
                        let base64 = image_base64::to_base64(image_path.as_str());
                        match File::create(&image_path_output) {
                            Ok(mut fi) => {
                                fi.write_all(base64.as_bytes())?;
                            }
                            Err(e) => {
                                if e.kind() == ErrorKind::NotFound {
                                    fs::create_dir("/home/sangulo/Imágenes/b65")?;
                                    let mut fi = File::create(&image_path_output)?;
                                    fi.write_all(base64.as_bytes())?;
                                }
                            }
                        };
                    }
                }
                _ => {}
            }
        }
    }
    Ok(())
}
