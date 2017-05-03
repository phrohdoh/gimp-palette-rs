extern crate gimp_palette;

fn main() {
    let colors = vec![ gimp_palette::Color { r: 0, g: 50, b: 255 } ];
    gimp_palette::create_gpl("test.gpl", &colors).expect("Failed to create test.gpl");
}
