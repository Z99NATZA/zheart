use std::io::{self, Write};

fn main() {
    let size = 100;
    let r: f32 = 0.4;
    let size2 = (size - 1) as f32;
    let mut t: f32 = 0.5;
    let chars = b" .:-+*#@";

    print!("\x1b[2J");

    loop {
        let mut zb = [f32::NEG_INFINITY; 100 * 40];      

        for row in 0..size {
            for col in 0..size {
                let x = ((col as f32 / size2) - 0.5) * 1.2;
                let y = (0.5 - (row as f32 / size2)) * 1.2;
                let a = 1.2 * y - ((2.0 * x.abs()) / 3.0);
                let inside = r.powi(2) - x.powi(2) - a.powi(2);
    
                if inside >= 0.0 {
                    let z = inside.sqrt() / (2.0 - y);
    
                    let mut zp = -z;

                    while zp < z {
                        let xpr = x * t.cos() - zp * t.sin();
                        let zpr = x * t.sin() + zp * t.cos();
    
                        zp += z / 12.0;
                    
                        let p = 1.0 + (zpr / 2.0);
                        let ux = ((xpr * p + 0.5) * 80.0 + 10.0).round() as i32;
                        let uy = ((-y * p + 0.5) * 39.0 + 2.0).round() as i32;

                        if (0..100).contains(&ux) && (0..40).contains(&uy) {
                            let idx = uy as usize * 100 + ux as usize;

                            if zpr > zb[idx] {
                                zb[idx] = zpr;
                            }
                        }
                    }
                }
            }
        }

        let mut d: [u8; 100 * 40] = [b' '; 100 * 40];

        let valid: Vec<f32> = zb.iter()
            .copied()
            .filter(|z| z.is_finite())
            .collect();

        let minz = valid.iter().copied().reduce(f32::min).unwrap_or(0.0);
        let maxz = valid.iter().copied().reduce(f32::max).unwrap_or(0.0);

        for row in 0..40 {
            for col in 0..100 {
                let idx = row * 100 + col;

                if zb[idx].is_finite() {
                    let range = maxz - minz;
                    let k = if range > 0.0 {
                        (((zb[idx] - minz) / range) * 7.0).round() as usize
                    } else {
                        0
                    };

                    d[idx] = chars[k.min(7)];
                } else {
                    d[idx] = b' ';
                }
            }
        }

        print!("\x1b[H");

        for row in 0..40 {
            let start = row * 100;
            let end = start + 100;

            println!("{}", std::str::from_utf8(&d[start..end]).unwrap());
        }

        io::stdout().flush().unwrap();

        t += 0.006;
    }
}
