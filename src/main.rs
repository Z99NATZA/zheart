fn main() {
    let r: f32 = 0.4;
    let size = 40;
    let sub_size: f32 = size as f32 - 1.0;

    for row in 0..size {
        for col in 0..size {
            let x = ((col as f32) / sub_size - 0.5) * 1.2;
            let y = (0.5 - (row as f32) / sub_size) * 1.2;

            let a = 1.2 * y - ((2.0 * x.abs()) / 3.0);
            let inside = r.powi(2) - x.powi(2) - a.powi(2);
            
            if inside >= 0.0 {
                let z = inside.sqrt() / (2.0 - y);
                
                if z > 0.15 {
                    print!(": ");
                }
                else {
                    print!(". ");
                }
            }
            else {
                print!("  ");
            }
        }
        println!();
    }
}
