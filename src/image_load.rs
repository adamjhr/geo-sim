
pub fn load_image(name: str) {
    let image = image::open("../images/" + name).unwrap();
}
