const IMAGE_HEIGHT: i64 = 256;
const IMAGE_WIDTH: i64 = 256;

fn main() {
    // Render

    // colors are in ascii
    println!("P3");

    // columns, rows
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);

    // max color
    println!("255");

    // RGB triplets
    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
            let r: f64 = i as f64 / (IMAGE_WIDTH - 1) as f64;
            let g: f64 = j as f64 / (IMAGE_WIDTH - 1) as f64;
            let b = 0.25;

            let ir = (255.999 * r) as i64;
            let ig = (255.999 * g) as i64;
            let ib = (255.999 * b) as i64;

            println!("{ir} {ig} {ib}");
        }
    }
}
