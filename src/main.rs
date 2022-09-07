use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer};

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let img = match clipboard.get_image() {
        Ok(img) => img,
        Err(_e) => {
            println!("Image in clipboard not found...");
            return;
        }
    };

    let image_buffer = ImageBuffer::from_raw(
        img.width.try_into().unwrap(),
        img.height.try_into().unwrap(),
        img.bytes.into_owned(),
    )
    .unwrap();

    let dyn_image = DynamicImage::ImageRgba8(image_buffer);
    let grey_image = dyn_image.to_luma8();
    let mut decoder = quircs::Quirc::default();
    let mut codes = decoder.identify(
        grey_image.width() as usize,
        grey_image.height() as usize,
        &grey_image,
    );

    let code = codes.next().unwrap().unwrap();
    let content = code.decode().unwrap();
    let str_content = std::str::from_utf8(&content.payload).unwrap();

    let mut result = String::new();

    for line in str_content.lines() {
        let line_vec: Vec<&str> = line.split("=").collect();
        let key = line_vec.first().unwrap().to_owned();

        match key {
            "Url" => {
                result.push_str("export");
                result.push_str(" ");
                result.push_str("WebServiceURL");
                result.push_str("=");
                result.push_str(line_vec.last().unwrap().to_owned());
                result.push_str(";");

                result.push_str("export");
                result.push_str(" ");
                result.push_str("BackendURL");
                result.push_str("=");
                result.push_str(line_vec.last().unwrap().to_owned());
                result.push_str(";");
            }
            "UserName" => {
                result.push_str("export");
                result.push_str(" ");
                result.push_str("DebugLogin");
                result.push_str("=");
                result.push_str(line_vec.last().unwrap().to_owned());
                result.push_str(";");
            }
            "Password" => {
                result.push_str("export");
                result.push_str(" ");
                result.push_str("DebugPwd");
                result.push_str("=");
                result.push_str("\"");
                result.push_str(line_vec.last().unwrap().to_owned());
                result.push_str("\"");
                result.push_str(";");
            }
            _ => {}
        }
    }
    println!("{}", result);
}
