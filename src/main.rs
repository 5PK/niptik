use clap::Parser;
use std::path::{Path, PathBuf};
use std::fs::File;
use std::io::{self, Read};
use std::fs;
use image::{GenericImageView, Rgb, ImageBuffer, RgbImage};

/// A program that stitches together half frame photos. From a folder of half frame photos, it will
/// stitch together images sequentially, or randomly
#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// The folder in which the half frame images are stored.
    #[arg(short, long)]
    path: PathBuf,

    /// The number of images to generate. Since we're stitching two images, for 1 image it will
    /// expect at least 2 jpegs in the folder, 2 images will expect 4 jpegs etc.
    #[arg(short, long, default_value_t = 1)]
    count: u8,

    /// Whether to stitch the Images randomly. If left out, it will stitch sequentially.
    #[arg(short, long)]
    random: bool
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    let path = args.path;

    // Verify the provided path exists and is a directory
    if !path.exists() {
        eprintln!("Error: Path does not exist.");
        std::process::exit(1);
    }
    if !path.is_dir() {
        eprintln!("Error: Path is not a directory.");
        std::process::exit(1);
    }

    let mut jpeg_files: Vec<String> = Vec::new();
    
    println!("UM HELLO");

    let expected_image_count = args.count * 2;

    if let Ok(entries) = fs::read_dir(path){
        for entry in entries{

            if jpeg_files.len() == expected_image_count as usize {
                break;
            }

            if let Ok(entry) = entry {
                let path = entry.path();
                if is_jpeg(&path)? {
                    jpeg_files.push(path.to_string_lossy().to_string());
                }

            }
        }
    }

    assert!(
        jpeg_files.len() == args.count as usize * 2,
        "Number of JPEGS provided does not meet required amount to generate {} images. Got {}.",
        args.count,
        jpeg_files.len()
    );

    for chunk in jpeg_files.chunks(2){

        if chunk.len() != 2{
            eprint!("what the hell");
            break;
        }
        
        let image1 = image::open(&chunk[0]).unwrap().to_rgb8();
        let image2 = image::open(&chunk[1]).unwrap().to_rgb8();

        let (width1, height1) = image1.dimensions();
        let (width2, height2) = image2.dimensions();

        println!("Image 1 dimensions: width = {}, height = {}", width1, height1);
        println!("Image 2 dimensions: width = {}, height = {}", width2, height2);

        // Dimensions of the new image
        let new_width = std::cmp::max(width1, width2);
        let new_height = height1 + height2 + 12;

        println!("New Image Dimensions {}, {}", new_width, new_height);

        // New imag
        let mut new_img: RgbImage = ImageBuffer::from_pixel(new_width, new_height, Rgb([0, 0, 0]));


        // Copy the first image into the new image
        for x in 0..width1 {
            for y in 0..height1 {

                new_img.put_pixel(x, y, *image1.get_pixel(x, y));
            }
        }

        // Copy the second image into the new image, starting after the 12px divider
        for x in 0..width2 {
            for y in 0..height2 {
                new_img.put_pixel(x, y + height1 + 12, *image2.get_pixel(x, y));
            }
        }
       
        println!("f1: {}", chunk[0]);
        println!("f2L {}", chunk[1]);
        
        // Get the filenames from the paths
        let filename1 = Path::new(&chunk[0]).file_name().unwrap().to_str().unwrap();
        let filename2 = Path::new(&chunk[1]).file_name().unwrap().to_str().unwrap();

        // Concatenate the filenames (removing extensions if needed)
        let _concat_filename = format!("{}_{}", filename1.trim_end_matches(".jpg"), filename2);


        new_img.save(_concat_filename).unwrap();
        

    }

    
    Ok(())

}

fn is_jpeg(file_path: &Path) -> io::Result<bool> {
    println!("OKKKK");
    let mut file = File::open(file_path)?;
    let mut buffer = [0,2];
    file.read_exact(&mut buffer)?;

    //JPEG files start with the bytes 0xFF and 0xD8
    Ok(buffer == [0xFF, 0xD8])
}

