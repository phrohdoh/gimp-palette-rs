use std::path::Path;
use std::io::Write;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl ToString for Color {
    fn to_string(&self) -> String {
        format!("{:3} {:3} {:3}", self.r, self.g, self.b)
    }
}

pub fn create_gpl<P: AsRef<Path>>(gpl_path: P, colors: &[Color]) -> Result<(), std::io::Error> {
  let s = create_string_from_colors(colors);
  let mut f = std::fs::OpenOptions::new()
    .write(true)
    .create(true)
    .open(gpl_path)?;

  f.write_all(s.as_bytes())
}

fn create_string_from_colors(colors: &[Color]) -> String {
    let mut s = format!("GIMP Palette\nName: test-palette\nColumns: {}\n",
                        colors.len());
    s.extend(colors.iter().map(|c| c.to_string() + "\n"));
    s
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let colors = vec![super::Color { r: 255, g:   0, b:   0 },
                          super::Color { r:   0, g: 255, b:   0 },
                          super::Color { r:   0, g:   0, b: 255 }];

        assert_eq!(super::create_string_from_colors(&colors),
                   r#"GIMP Palette
Name: test-palette
Columns: 3
255   0   0
  0 255   0
  0   0 255
"#);
    }
}
