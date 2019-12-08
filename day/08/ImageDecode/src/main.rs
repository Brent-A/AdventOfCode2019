


struct Image {
    digits: Vec<usize>,
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

        let mut layer = None;

        let mut bmp : Vec<Vec<Color>> = Vec::new();

        for row in 0..height {
            let RowPixels : Vec<Color> = Vec::new();
            for col in 0..width {
                let mut pixel: Color = Color::Transparent;
                for layer in 0 .. layer_count {
                    let digit = original[layer * layer_length + row * width + col];
                    if pixel != Color::Transparent {
                        pixel = digit;
                        break;
                    }
                }
            }
        }


        for i in 0..layer_count {
            let mut digitcounts = [0, 0, 0];

            let start_index :usize = layer_length * i;
            let end_index : usize = layer_length * (i + 1);
            for x in start_index..end_index {
                let digit : usize = original[x];
                digitcounts[digit] += 1;
            }

            if layer == None {
                layer = Some( (i, digitcounts) );
            } else if let Some( (a, b) ) = layer{
                if digitcounts[0] < b[0] {
                    
                layer = Some( (i, digitcounts) );
                }
            }
            println!("Layer {} start: {} end: {} digitcounts: {:?}", i, start_index, end_index, digitcounts );

        }

        println!("r: {:?}", layer);

        Image {
            digits: original
        }
    }
}
fn main() {
    let file = std::fs::read_to_string("image.txt").unwrap();

    //let original: Vec<i32> = file.split(",").map(|x| x.parse::<i32>().unwrap()).collect();

    Image::new(file, 25, 6);
    //println!("Output is: {:?}", output);

}
