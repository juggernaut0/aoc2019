pub fn run1(input: Vec<String>) -> u32 {
    let image = Image::from_string_with_size(&input[0], 25, 6);
    image.layers.iter()
        .min_by_key(|it| num_of(it, 0))
        .map(|layer| num_of(layer, 1) * num_of(layer, 2))
        .unwrap()
}

pub fn run2(input: Vec<String>) {
    let image = Image::from_string_with_size(&input[0], 25, 6);
    image.print();
}

type Layer = Vec<Vec<u32>>;
struct Image {
    layers: Vec<Layer>,
    width: usize,
    height: usize,
}

impl Image {
    fn from_string_with_size(s: &str, width: usize, height: usize) -> Image {
        let mut nums = s.chars().map(|it| it.to_digit(10).unwrap());
        let mut layers = Vec::new();
        'outer: loop {
            let mut layer = Vec::new();
            for _y in 0..height {
                let mut row = Vec::new();
                for _x in 0..width {
                    match nums.next() {
                        Some(n) => row.push(n),
                        None => break 'outer
                    }
                }
                layer.push(row)
            }
            layers.push(layer)
        }
        Image { layers, width, height }
    }

    fn pixel_at(&self, x: usize, y: usize) -> u32 {
        self.layers.iter().map(|it| it[y][x]).find(|it| *it != 2).unwrap_or(2)
    }

    fn print(&self) {
        for y in 0..self.height {
            let mut s = String::new();
            for x in 0..self.width {
                let pixel = self.pixel_at(x, y);
                s.push(if pixel == 0 { ' ' } else { '#' });
            }
            println!("{}", s);
        }
    }
}

fn num_of(layer: &Layer, digit: u32) -> u32 {
    layer.iter().flat_map(|it| it.iter()).filter(|d| **d == digit).count() as u32
}
