use mindr;

// TODO: think through the logic to add initial config after creating new file
// TODO: probably I want to parse config inside fully and get custom struct config in return
// TODO: write integration case for entire process of creating config etc., but testing each field
// is probably redundant as we will have a resulting struct
fn main() {
    let config = mindr::get_configuration();
    println!("{:?}", config);
}
