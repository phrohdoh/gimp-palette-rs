use std::path::Path;
use std::io::Write;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Palette {
    _name: String,
    _colors: Vec<Color>,
}

impl Palette {
    /// Creates a new named Palette from a non-zero-length collection of Colors.
    ///
    /// Passing a zero-length collection will result in an Err(String)
    pub fn new<S: ToString>(name: S, colors: Vec<Color>) -> Result<Self, String> {
        match colors.len() {
            0 => Err(String::from("Palettes must have at least 1 color")),
            _ => Ok(Palette {
                _name: name.to_string(),
                _colors: colors,
            }),
        }
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        &self._name
    }

    pub fn get_colors<'a>(&'a self) -> &'a [Color] {
        &self._colors
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), std::io::Error> {
        let header = format!("GIMP Palette\nName: {}\nColumns: {}\n", self._name, self._colors.len());
        let colors_string = create_string_from_colors(&self._colors);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        let final_string = header + &colors_string;
        f.write_all(final_string.as_bytes())
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:3} {:3} {:3}", self.r, self.g, self.b)
    }
}

fn create_string_from_colors(colors: &[Color]) -> String {
    colors.iter().map(|c| c.to_string() + "\n").collect::<String>()
}

#[cfg(test)]
mod tests {
    #[test]
    fn colorless_palette_err() {
        assert!(super::Palette::new("Failure", vec![]).is_err());
    }

    #[test]
    fn colors_1() {
        let colors = vec![super::Color { r: 255, g: 0, b: 0 }];

        assert_eq!(super::create_string_from_colors(&colors),
                   "255   0   0\n");
    }

    #[test]
    fn colors_3() {
        let colors = vec![super::Color { r: 255, g: 0, b: 0 },
                          super::Color { r: 0, g: 255, b: 0 },
                          super::Color { r: 0, g: 0, b: 255 }];

        assert_eq!(super::create_string_from_colors(&colors),
                   "255   0   0\n  0 255   0\n  0   0 255\n");
    }
}
