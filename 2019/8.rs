use std::io;
use std::io::Write;

const WIDTH: usize = 25;
const HEIGHT: usize = 6;

fn main() -> io::Result<()> {
    let mut input = String::new();

    match io::stdin().read_line(&mut input) {
        Ok(_n) => (),
        Err(error) => println!("error: {}", error),
    }

    let raw = get_raw_from_string(input.trim_end().to_string());

    let layers = get_layers_from_raw(raw, WIDTH, HEIGHT);

    let layers_with_num_zeros = layers.iter().zip(layers.iter().map(|l| l.iter().filter(|&&i| i == 0).count()));

    let mut least_zeros_layer = &vec![];
    let mut least_zeros = None;

    for (layer, num) in layers_with_num_zeros {
        match least_zeros {
            None => {
                least_zeros_layer = layer;
                least_zeros = Some(num);
            },
            Some(z) => {
                least_zeros_layer = if num < z { layer } else { least_zeros_layer };
                least_zeros = Some(if num < z { num } else { z });
            },
        }
    }

    let num_ones = least_zeros_layer.iter().filter(|&&i| i == 1).count();
    let num_twos = least_zeros_layer.iter().filter(|&&i| i == 2).count();

    println!("num_ones * num_twos: {}", num_ones * num_twos);

    let mut result_image = vec![];

    for i in 0..WIDTH*HEIGHT {
        let mut pixel = 2;
        let mut j = 0;

        while pixel == 2 {
            pixel = layers[j][i];

            j += 1;
        }

        result_image.push(pixel);
    }

    println!("result_image: {:?}", result_image);

    print_image(result_image);

    Ok(())
}

fn print_image(image:Vec<i32>) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let s = if image[WIDTH*y + x] == 0 { " " } else { "#"};

            io::stdout().write_all(s.as_bytes());
        }
        io::stdout().write_all(b"\n");
    }
    io::stdout().flush();
}

fn get_raw_from_string(input:String) -> Vec<i32> {
    input.chars().map(|c| c.to_digit(10).unwrap() as i32).collect()
}

fn get_layers_from_raw(raw: Vec<i32>, width:usize, height:usize) -> Vec<Vec<i32>> {
    (0..raw.len() / (width * height)).map(|i| raw[i*width*height..(i+1)*width*height].to_vec()).collect()
}

#[cfg(test)]
mod tests {
    use *;

    #[test]
    fn test_execute_program() {
    }
}
