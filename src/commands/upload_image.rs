use fs_err as fs;

use image::{codecs::png::PngEncoder, DynamicImage, GenericImage, GenericImageView, Rgba};

use std::borrow::Cow;

use crate::{
    alpha_bleed::alpha_bleed,
    auth_cookie::get_auth_cookie,
    options::{GlobalOptions, UploadImageOptions},
    roblox_web_api::{ImageUploadData, RobloxApiClient},
};

#[allow(dead_code)]
pub fn make_opaque(img: &mut DynamicImage) {
    for y in 0..img.height() {
        for x in 0..img.width() {
            let pixel = img.get_pixel(x, y);
            if pixel[3] != 255 {
                let opaque_pixel = Rgba([pixel[0], pixel[1], pixel[2], 255]);
                img.put_pixel(x, y, opaque_pixel)
            }
        }
    }
}

pub fn upload_image(global: GlobalOptions, options: UploadImageOptions) {
    let auth = global
        .auth
        .or_else(get_auth_cookie)
        .expect("no auth cookie found");

    let image_data = fs::read(options.path).expect("couldn't read input file");

    let mut img = image::load_from_memory(&image_data).expect("couldn't load image");

    alpha_bleed(&mut img, 1);
    make_opaque(&mut img);

    match img.save("output.png") {
        Ok(_) => println!("Successfully saved"),
        Err(_) => println!("Failed to save"),
    }

    // let (width, height) = img.dimensions();

    // let mut encoded_image: Vec<u8> = Vec::new();
    // PngEncoder::new(&mut encoded_image)
    //     .encode(&img.to_bytes(), width, height, img.color())
    //     .unwrap();

    // let mut client = RobloxApiClient::new(Some(auth));

    // let upload_data = ImageUploadData {
    //     image_data: Cow::Owned(encoded_image.to_vec()),
    //     name: &options.name,
    //     description: &options.description,
    //     group_id: None,
    // };

    // let response = client
    //     .upload_image(upload_data)
    //     .expect("Roblox API request failed");

    // eprintln!("Image uploaded successfully!");
    // println!("{}", response.backing_asset_id);
}
