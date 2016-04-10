extern crate freetype;

#[test]
fn it_works() {
    freetype::Library::init().unwrap();
}
