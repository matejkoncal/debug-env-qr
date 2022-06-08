use arboard::Clipboard;
use image::{DynamicImage, ImageBuffer};
use string_builder::Builder;

fn main() {
    let mut clipboard = Clipboard::new().unwrap();
    let img = match clipboard.get_image() {
        Ok(img) => img,
        Err(_e) => {
            println!("Image in clipboard not found...");
            return;
        }
    };

    let decoder = bardecoder::default_decoder();

    let image_buffer = ImageBuffer::from_raw(
        img.width.try_into().unwrap(),
        img.height.try_into().unwrap(),
        img.bytes.into_owned(),
    )
    .unwrap();

    let dyn_image = DynamicImage::ImageRgba8(image_buffer);
    let qr_decoder_result = decoder.decode(&dyn_image);

    let mut str_builder = Builder::default();
    for line in qr_decoder_result.first().unwrap().as_ref().unwrap().lines() {
        let line_vec: Vec<&str> = line.split("=").collect();
        let key = line_vec.first().unwrap().to_owned();

        match key {
            "Url" => {
                str_builder.append("export");
                str_builder.append(" ");
                str_builder.append("WebServiceURL");
                str_builder.append("=");
                str_builder.append(line_vec.last().unwrap().to_owned());
                str_builder.append(";");

                str_builder.append("export");
                str_builder.append(" ");
                str_builder.append("BackendURL");
                str_builder.append("=");
                str_builder.append(line_vec.last().unwrap().to_owned());
                str_builder.append(";");
            }
            "UserName" => {
                str_builder.append("export");
                str_builder.append(" ");
                str_builder.append("DebugLogin");
                str_builder.append("=");
                str_builder.append(line_vec.last().unwrap().to_owned());
                str_builder.append(";");
            }
            "Password" => {
                str_builder.append("export");
                str_builder.append(" ");
                str_builder.append("DebugPwd");
                str_builder.append("=");
                str_builder.append("\"");
                str_builder.append(line_vec.last().unwrap().to_owned());
                str_builder.append("\"");
                str_builder.append(";");
            }
            _ => {}
        }
    }
    println!("{}", str_builder.string().unwrap());
}
