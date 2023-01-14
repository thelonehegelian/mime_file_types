use mime::{Mime, APPLICATION_OCTET_STREAM};

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
}
