

#[derive(Debug)]
struct Image {
    bmp: Vec<Vec<Color>>,
}

use std::convert::TryInto;

fn get_value(c: char) -> Color {
    match c {
        '0' => Color::Black,
        '1' => Color::White,
        '2' => Color::Transparent,
        _ => { panic!("unexpected"); }
    }
}

#[derive(PartialEq, Debug, Copy, Clone)]
enum Color {
    Black = 0,
    White = 1,
    Transparent  = 2,
}

impl Image {
    pub fn new(s: String, width: usize, height: usize) -> Self {

        let original: Vec<Color> = s.chars().map(|x| get_value(x)).collect();
        let layer_length = width * height;
        let layer_count : usize = original.len() / layer_length;

        let mut bmp : Vec<Vec<Color>> = Vec::new();

        for row in 0..height {
            let mut RowPixels : Vec<Color> = Vec::new();
            for col in 0..width {
                let mut pixel: Color = Color::Transparent;
                for layer in 0 .. layer_count {
                    let digit = original[layer * layer_length + row * width + col];
                    if digit != Color::Transparent {
                        pixel = digit;
                        break;
                    }
                }
                RowPixels.push(pixel);
            }
            bmp.push(RowPixels);
        }

        Image {
            bmp: bmp
        }
    }
}
fn main() {
    let file = std::fs::read_to_string("image.txt").unwrap();

    //let original: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    let i = Image::new(file, 25, 6);
    println!("Output is: {:?}", i);

    for row in i.bmp {
        for col in row {
            if col == Color::Black {
                print!("X");
            } else {
                print!(" ");
            }
        }
        println!("");
    }

}
