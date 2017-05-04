extern crate gimp_palette;
use gimp_palette::{Palette, NewPaletteError};

fn main() {
    match std::env::args().nth(1) {
        None => println!("Please provide a path to a gpl file"),
        Some(file_path) => {
            let pal = match Palette::read_from_file(&file_path) {
                Ok(p) => p,
                Err(e) => match e {
                    NewPaletteError::NoColors => unreachable!(),
                    NewPaletteError::InvalidData { line_num, val } => panic!("Line {} has invalid data: {}", line_num, val),
                    NewPaletteError::IoErr(io_err) => panic!("{}", io_err),
                }
            };

            println!("Found {} color(s) in {}", pal.get_colors().len(), file_path);
        }
    }
}
