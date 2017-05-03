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
    pub fn new<S: ToString>(name: S, colors: Vec<Color>) -> Self {
        Palette {
            _name: name.to_string(),
            _colors: colors,
        }
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        &self._name
    }

    pub fn get_colors<'a>(&'a self) -> &'a [Color] {
        &self._colors
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), std::io::Error> {
        let s = create_string_from_colors(&self._colors);
        let mut f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open(file_path)?;

        f.write_all(s.as_bytes())
    }
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:3} {:3} {:3}", self.r, self.g, self.b)
    }
}

fn create_string_from_colors(colors: &[Color]) -> String {
    let mut s = format!("GIMP Palette\nName: nameless-palette\nColumns: {}\n",
                        colors.len());
    s.extend(colors.iter().map(|c| c.to_string() + "\n"));
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let colors = vec![super::Color { r: 255, g: 0, b: 0 },
                          super::Color { r: 0, g: 255, b: 0 },
                          super::Color { r: 0, g: 0, b: 255 }];

        assert_eq!(super::create_string_from_colors(&colors),
                   r#"GIMP Palette
Name: nameless-palette
Columns: 3
255   0   0
  0 255   0
  0   0 255
"#);
    }
}
