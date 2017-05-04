use std::path::Path;
use std::io::{BufRead, BufReader, Write};
use std::fs::File;

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

pub struct Palette {
    _name: String,
    _colors: Vec<Color>,
}

pub enum NewPaletteError {
    NoColors,
    InvalidData { line_num: usize, val: String },
    IoErr(std::io::Error),
}

impl From<std::io::Error> for NewPaletteError {
    fn from(e: std::io::Error) -> Self {
        NewPaletteError::IoErr(e)
    }
}

impl Palette {
    /// Creates a new named Palette from a non-zero-length collection of Colors.
    ///
    /// Passing a zero-length collection will result in an Err(NewPaletteError::NoColors)
    pub fn new<S: ToString>(name: S, colors: Vec<Color>) -> Result<Self, NewPaletteError> {
        match colors.len() {
            0 => Err(NewPaletteError::NoColors),
            _ => {
                Ok(Palette {
                       _name: name.to_string(),
                       _colors: colors,
                   })
            }
        }
    }

    pub fn read_from_file<P: AsRef<Path>>(file_path: P) -> Result<Self, NewPaletteError> {
        let f = File::open(&file_path)?;
        let reader = BufReader::new(f);

        fn is_comment(s: &str) -> bool {
            s.chars().skip_while(|c| c.is_whitespace()).next() == Some('#')
        }

        let mut colors = vec![];
        let mut name = String::new();
        //let mut color_count = 0;

        let mut lines = reader.lines().enumerate();
        while let Some((index, Ok(line))) = lines.next() {
            if is_comment(&line) || line.trim().len() == 0 {
                continue;
            }

            let line_num = index + 1;

            if line_num == 1 {
                if line != "GIMP Palette" {
                    return Err(NewPaletteError::InvalidData {
                                   line_num,
                                   val: line,
                               });
                }

                continue;
            } else if line_num == 2 {
                if !line.starts_with("Name:") {
                    return Err(NewPaletteError::InvalidData {
                                   line_num,
                                   val: line,
                               });
                }

                name = line[4..].trim().to_string();
                continue;
            } else if line_num == 3 {
                if !line.starts_with("Columns:") {
                    return Err(NewPaletteError::InvalidData {
                                   line_num,
                                   val: line,
                               });
                }

                // TODO: Handle this value.
                // - Should we only read `color_count` colors, or
                // - Err if color_count != (line count - 3 - comments), or
                // - Explicitly ignore this value
                //color_count = line[4..].trim().to_string();
                continue;
            }

            let mut split = line.split_whitespace();
            match (split.next(), split.next(), split.next()) {
                (Some(r_str), Some(g_str), Some(b_str)) => {
                    let r =
                        r_str
                            .parse::<u8>()
                            .expect(&format!("Failed to parse line {}'s r into a byte", line_num));
                    let g =
                        g_str
                            .parse::<u8>()
                            .expect(&format!("Failed to parse line {}'s g into a byte", line_num));
                    let b =
                        b_str
                            .parse::<u8>()
                            .expect(&format!("Failed to parse line {}'s b into a byte", line_num));
                    colors.push(Color { r, g, b });
                }
                _ => {
                    return Err(NewPaletteError::InvalidData {
                                   line_num,
                                   val: line.clone(),
                               })
                }
            }
        }

        Ok(Self {
               _name: name,
               _colors: colors,
           })
    }

    pub fn get_name<'a>(&'a self) -> &'a str {
        &self._name
    }

    pub fn get_colors<'a>(&'a self) -> &'a [Color] {
        &self._colors
    }

    pub fn write_to_file<P: AsRef<Path>>(&self, file_path: P) -> Result<(), std::io::Error> {
        let header = format!("GIMP Palette\nName: {}\nColumns: {}\n",
                             self._name,
                             self._colors.len());
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
    colors
        .iter()
        .map(|c| c.to_string() + "\n")
        .collect::<String>()
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

        assert_eq!(super::create_string_from_colors(&colors), "255   0   0\n");
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
