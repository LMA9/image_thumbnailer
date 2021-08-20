use std::fs::{create_dir_all, read_dir, remove_file};
use std::path::Path;
use std::process::exit;
use image::io::Reader;
use image::DynamicImage;
use std::thread;
use std::time::Duration;
use std::env;

const HELP_STRING: &str = "
Usage: `image_thumbnailer Optional['source_path', 'target_path', [thumbnails_sizes]]
Example: `image_thumbnailer images/wallpapers thumbnails 240,480,960`";

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

fn create_thumbnails(image_path: &str, target_path: String, sizes: Vec<u32>) {
    let img = Reader::open(image_path).unwrap().decode().unwrap();
    let image_name = Path::new(image_path).file_name().unwrap().to_str().unwrap();
    
    for size in sizes.iter() {
        create_thumbnail(&img, image_name, *size, &target_path);
    }

    remove_file(image_path).unwrap()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let images_path;
    let target_path;
    let sizes: Vec<u32>;

    match args.get(1) {
        Some(arg) => {
            let first_arg = arg.as_str();
            if first_arg == "help" {
                println!("{}", HELP_STRING);
                exit(0);
            }
            images_path = first_arg
        },
        None => images_path = "images"
    }

    match args.get(2) {
        Some(arg) => target_path = arg.as_str(),
        None => target_path = "thumbnails"
    }

    match args.get(3) {
        Some(sizes_string) => {
            sizes = sizes_string.split(',').map(|s| s.parse().unwrap()).collect()
        },
        None => sizes = vec![240, 480]
    }

    println!("Thumbnailer was started.\nSource path: {}\nTarget path: {}", images_path, target_path);

    loop {
        if Path::new(images_path).exists() {
            for res in read_dir(images_path).unwrap() {
                let target = target_path.to_string();
                let sizes = sizes.clone();
                match res {
                    Ok(entry) => {
                        let entry_path = entry.path();
                        if entry_path.is_file() {
                            thread::spawn(move || {
                                let file_path = entry_path.to_str().unwrap();
                                create_thumbnails(file_path, target, sizes)
                            });
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
