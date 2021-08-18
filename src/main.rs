use std::fs::{create_dir_all, read_dir, remove_file};
use std::path::Path;
use image::io::Reader;
use image::DynamicImage;
use std::thread;
use std::time::Duration;
use std::env;


fn create_thumbnail(image: &DynamicImage, image_name: &str, size: u32, target_path: &String) {
    let thumbnails_path = format!("{}/{}", target_path, size);
    let new_image_path = format!("{}/{}", thumbnails_path, image_name);

    if !Path::new(&new_image_path).exists() {
        let new_img = image.thumbnail(size, size);
        
        
        if !Path::new(&thumbnails_path).exists() {
            create_dir_all(thumbnails_path).unwrap();
        }
        
        new_img.save(new_image_path).unwrap()
    }
}

fn create_thumbnails(image_path: &str, target_path: String) {
    let img = Reader::open(image_path).unwrap().decode().unwrap();
    let image_name = Path::new(image_path).file_name().unwrap().to_str().unwrap();
    
    create_thumbnail(&img, image_name, 240, &target_path);
    create_thumbnail(&img, image_name, 480, &target_path);

    remove_file(image_path).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let images_path;
    let target_path;

    match args.get(1) {
        Some(source) => images_path = source.as_str(),
        None => images_path = "images"
    }

    match args.get(2) {
        Some(source) => target_path = source.as_str(),
        None => target_path = "thumbnails"
    }

    println!("Thumbnailer was started.\nSource path: {}\nTarget path: {}", images_path, target_path);

    loop {
        if Path::new(images_path).exists() {
            for res in read_dir(images_path).unwrap() {
                let target = target_path.to_string();
                match res {
                    Ok(entry) => {
                        if entry.path().is_file() {
                            thread::spawn(move || { create_thumbnails(entry.path().to_str().unwrap(), target) });
                        }
                    }
                    Err(_e) => {}
                }
            }
            thread::sleep(Duration::from_secs(5))

        } else {
            thread::sleep(Duration::from_secs(30))
        }
    }    
}
