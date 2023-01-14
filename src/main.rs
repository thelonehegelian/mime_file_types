use mime::{Mime, APPLICATION_OCTET_STREAM, IMAGE_JPEG, TEXT_PLAIN, VIDEO};

fn main() {
    /****************************************
     * check for valid and invalid mime types
     ****************************************/

    let invalid_mime = "I N V A L I D";
    let default_mime: Mime = invalid_mime
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);
    println!("{:?}", default_mime);

    let valid_mime = "application/json";
    let default_mime: Mime = valid_mime
        .parse::<Mime>()
        .unwrap_or(APPLICATION_OCTET_STREAM);
    println!("{:?}", default_mime);

    /****************************************
     * GET MIME TYPE FROM A FILE EXTENSION
     ****************************************/
    let files = vec!["image.jpg", "text.json", "another.png"];
    for file in files {
        let mime = find_mime_type(file);
        println!("{:?}", mime);
    }
    fn find_mime_type(file: &str) -> Mime {
        let parts = file.split('.').collect::<Vec<&str>>();

        let result = match parts.last() {
            Some(ext) => match *ext {
                "jpg" => mime::IMAGE_JPEG,
                "png" => mime::IMAGE_PNG,
                "json" => mime::APPLICATION_JSON,
                _ => mime::TEXT_PLAIN,
            },
            None => APPLICATION_OCTET_STREAM,
        };
        return result;
    }
}
