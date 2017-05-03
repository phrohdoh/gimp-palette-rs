extern crate gimp_palette;

fn main() {
    let colors = vec![ gimp_palette::Color { r: 0, g: 50, b: 255 } ];
    println!("{}", gimp_palette::create_string_from_colors(&colors));
}
