fn main() {
    use minifb::{Key, Window, WindowOptions};
    use image::{ImageBuffer, RgbImage};

    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const WHITE: u32 = 0xFFFFFF;
    const GREEN: u32 = 0x00FF00;

    #[derive(Clone, Copy)]
    struct Point {
        x: i32,
        y: i32,
    }

    fn fill_polygon(buffer: &mut [u32], points: &[Point], color: u32, hole: Option<&[Point]>) {
        let mut ymin = i32::MAX;
        let mut ymax = i32::MIN;
        for p in points {
            ymin = ymin.min(p.y);
            ymax = ymax.max(p.y);
        }

        for y in ymin..=ymax {
            let mut intersecciones = Vec::new();
            for i in 0..points.len() {
                let p1 = points[i];
                let p2 = points[(i + 1) % points.len()];
                if (p1.y <= y && p2.y > y) || (p2.y <= y && p1.y > y) {
                    let x = p1.x + (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y);
                    intersecciones.push(x);
                }
            }

            intersecciones.sort();

            for pair in intersecciones.chunks(2) {
                if pair.len() == 2 {
                    let (start, end) = (pair[0], pair[1]);
                    for x in start..=end {
                        if x >= 0 && x < WIDTH as i32 && y >= 0 && y < HEIGHT as i32 {
                            let inside_hole = if let Some(hole_pts) = hole {
                                point_in_polygon(Point { x, y }, hole_pts)
                            } else {
                                false
                            };

                            if !inside_hole {
                                buffer[y as usize * WIDTH + x as usize] = color;
                            }
                        }
                    }
                }
            }
        }
    }

    fn point_in_polygon(p: Point, poly: &[Point]) -> bool {
        let mut inside = false;
        let mut j = poly.len() - 1;
        for i in 0..poly.len() {
            let pi = poly[i];
            let pj = poly[j];
            if (pi.y > p.y) != (pj.y > p.y) &&
                p.x < (pj.x - pi.x) * (p.y - pi.y) / (pj.y - pi.y) + pi.x {
                inside = !inside;
            }
            j = i;
        }
        inside
    }

    fn save_buffer_as_png(buffer: &[u32], filename: &str) {
        let mut img: RgbImage = ImageBuffer::new(WIDTH as u32, HEIGHT as u32);
        for (i, pixel) in buffer.iter().enumerate() {
            let x = (i % WIDTH) as u32;
            let y = (i / WIDTH) as u32;
            let r = ((pixel >> 16) & 0xFF) as u8;
            let g = ((pixel >> 8) & 0xFF) as u8;
            let b = (pixel & 0xFF) as u8;
            img.put_pixel(x, y, image::Rgb([r, g, b]));
        }
        img.save(filename).unwrap();
    }

    // --- Inicio ejecución ---
    let mut buffer = vec![WHITE; WIDTH * HEIGHT];

    let poly1 = vec![
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383)
    ];
    let convert = |v: Vec<(i32, i32)>| -> Vec<Point> {
        v.into_iter().map(|(x, y)| Point { x, y }).collect()
    };

    let points_poly1 = convert(poly1);
    fill_polygon(&mut buffer, &points_poly1, GREEN, None);

    let mut window = Window::new("Polígono Verde", WIDTH, HEIGHT, WindowOptions::default()).unwrap();
    while window.is_open() && !window.is_key_down(Key::Escape) {
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }

    save_buffer_as_png(&buffer, "poligono_verde.png");
}
