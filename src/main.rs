mod framebuffer;
mod bmp;
mod color;
mod line;

use framebuffer::FrameBuffer;
use color::Color;
use line::draw_line;

fn fill_polygon(frame_buffer: &mut FrameBuffer, points: &[(i32, i32)], color: &Color) {
    let mut nodes = Vec::new();
    let mut min_y = points[0].1;
    let mut max_y = points[0].1;

    for point in points {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    for y in min_y..=max_y {
        nodes.clear();

        let mut j = points.len() - 1;
        for i in 0..points.len() {
            if (points[i].1 < y && points[j].1 >= y) || (points[j].1 < y && points[i].1 >= y) {
                let x = points[i].0 + (y - points[i].1) * (points[j].0 - points[i].0) / (points[j].1 - points[i].1);
                nodes.push(x);
            }
            j = i;
        }

        nodes.sort();

        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    frame_buffer.set_pixel(x as usize, y as usize, color);
                }
            }
        }
    }
}

fn point_polygon(point: (i32, i32), polygon: &[(i32, i32)]) -> bool {
    let (x, y) = point;
    let mut inside = false;
    let mut j = polygon.len() - 1;
    for i in 0..polygon.len() {
        let (xi, yi) = polygon[i];
        let (xj, yj) = polygon[j];
        if ((yi > y) != (yj > y)) && (x < (xj - xi) * (y - yi) / (yj - yi) + xi) {
            inside = !inside;
        }
        j = i;
    }
    inside
}

fn polygon_perforation(fb: &mut FrameBuffer, outer: &[(i32, i32)], hole: &[(i32, i32)], color: &Color) {
    let mut nodes = Vec::new();
    let mut min_y = outer[0].1;
    let mut max_y = outer[0].1;

    for point in outer {
        if point.1 < min_y {
            min_y = point.1;
        }
        if point.1 > max_y {
            max_y = point.1;
        }
    }

    for y in min_y..=max_y {
        nodes.clear();

        let mut j = outer.len() - 1;
        for i in 0..outer.len() {
            if (outer[i].1 < y && outer[j].1 >= y) || (outer[j].1 < y && outer[i].1 >= y) {
                let x = outer[i].0 + (y - outer[i].1) * (outer[j].0 - outer[i].0) / (outer[j].1 - outer[i].1);
                nodes.push(x);
            }
            j = i;
        }

        nodes.sort();

        for n in (0..nodes.len()).step_by(2) {
            if n + 1 < nodes.len() {
                for x in nodes[n]..=nodes[n + 1] {
                    if !point_polygon((x, y), hole) {
                        fb.set_pixel(x as usize, y as usize, color);
                    }
                }
            }
        }
    }
}


fn main() {
    let mut frame_buffer = FrameBuffer::new(800, 600);

    let polygon_1 = [
        (165, 380), (185, 360), (180, 330), (207, 345), (233, 330),
        (230, 360), (250, 380), (220, 385), (205, 410), (193, 383),
    ];
    
    let polygon_2 = [
        (321, 335), (288, 286), (339, 251), (374, 302),
    ];

    let polygon_3 = [
        (377, 249), (411, 197), (436, 249),
    ];

    let polygon_4 = [
        (413, 177), (448, 159), (502, 88), (553, 53), (535, 36),
        (676, 37), (660, 52), (750, 145), (761, 179), (672, 192),
        (659, 214), (615, 214), (632, 230), (580, 230),
        (597, 215), (552, 214), (517, 144), (466, 180),
    ];

    let polygon_5 = [
        (682, 175), (708, 120), (735, 148), (739, 170),
    ];


    //Primer poligono
    fill_polygon(&mut frame_buffer, &polygon_1, &Color::YELLOW);

    for i in 0..polygon_1.len() {
        let (x0, y0) = polygon_1[i];
        let (x1, y1) = polygon_1[(i + 1) % polygon_1.len()];
        draw_line(&mut frame_buffer, x0, y0, x1, y1, &Color::WHITE);
    }

    // Segundo poligono
    fill_polygon(&mut frame_buffer, &polygon_2, &Color::BLUE);

    for i in 0..polygon_2.len() {
        let (x0, y0) = polygon_2[i];
        let (x1, y1) = polygon_2[(i + 1) % polygon_2.len()];
        draw_line(&mut frame_buffer, x0, y0, x1, y1, &Color::WHITE);
    }

    //Tercer poligono
    fill_polygon(&mut frame_buffer, &polygon_3, &Color::RED);

    for i in 0..polygon_3.len() {
        let (x0, y0) = polygon_3[i];
        let (x1, y1) = polygon_3[(i + 1) % polygon_3.len()];
        draw_line(&mut frame_buffer, x0, y0, x1, y1, &Color::WHITE);
    }

    //Cuarto Poligono
    polygon_perforation(&mut frame_buffer, &polygon_4, &polygon_5, &Color::GREEN);

    for i in 0..polygon_4.len() {
        let (x0, y0) = polygon_4[i];
        let (x1, y1) = polygon_4[(i + 1) % polygon_4.len()];
        draw_line(&mut frame_buffer, x0, y0, x1, y1, &Color::WHITE);
    }

    //Quinto poligono
    for i in 0..polygon_5.len() {
        let (x0, y0) = polygon_5[i];
        let (x1, y1) = polygon_5[(i + 1) % polygon_5.len()];
        draw_line(&mut frame_buffer, x0, y0, x1, y1, &Color::WHITE);
    }


    frame_buffer.save_as_bmp("out.bmp");
}