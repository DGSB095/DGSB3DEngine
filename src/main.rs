mod objects;
mod ray_casting;
mod math_lib;
mod Camera;

use math_lib::dot;
use objects::*;
use ray_casting::*;
use minifb::{Window, WindowOptions, Key};

fn main() {
    let mut flag = false;
    let mut frames:u128 = 0;
    let mut camera = Camera::Camera::new(0.0, 0.0, 0.0, 60.0);
    let move_speed = 0.1;
    let camera_origin:Vertex = Vertex::new(0f64,0f64,0f64);
    let DefaultColor:u32 = (192 << 24) | (192 << 16) | (192 << 8) | 255;
    let mut Vertecies:Vec<objects::Vertex> = Vec::new();
    let mut Faces:Vec<objects::Face> = Vec::new();
    let fov = 90.0f64;
    let width = 1900;
    let height = 1000;
    let mut aspect_ratio = width as f64 / height as f64;
    let scale = (fov.to_radians() / 2.0).tan();
    let mut buffer: Vec<u32> = vec![0; width * height];
    let spheres = vec![
        Sphere::new(Vertex::new(0.0, 0.0, 5.0), 0.5, 0xFF0000),
        Sphere::new(Vertex::new(1.0, 0.0, 6.0), 0.5, 0x00FF00),
        Sphere::new(Vertex::new(-1.0, 0.0, 6.0), 0.5, DefaultColor),
        Sphere::new(Vertex::new(0.0, 3.0, 5.0), 0.5, 0xEA0400),
        Sphere::new(Vertex::new(1.0, 0.0, 9.0), 0.5, 0xA0CF00),
        Sphere::new(Vertex::new(1.0, -2.0, 6.0), 1.0, 0xFC3A00),
        Sphere::new(Vertex::new(2.0, 0.0, 6.0), 0.5, 0xFF6E00),
    ];

    let mut window = Window::new(
        "Rust Window",
        width,
        height,
        WindowOptions{
            resize: true,
            borderless:true,
            ..WindowOptions::default()
        })
        .unwrap();

    let mut last_mouse = window.get_mouse_pos(minifb::MouseMode::Discard).unwrap_or((0.0, 0.0));
    let mouse_sensitivity = 0.002;


    let mut light_dir = Vertex::new(-1.0, 1.0, -1.0);
    let len = (light_dir.x * light_dir.x + light_dir.y * light_dir.y + light_dir.z * light_dir.z).sqrt();
    light_dir.x /= len;
    light_dir.y /= len;
    light_dir.z /= len;
    window.set_target_fps(120);
    window.set_cursor_visibility(false);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        let speed = 0.25;
        let yaw = camera.yaw;
        let forward = Vertex::new(yaw.sin(), 0.0, yaw.cos());
        let right   = Vertex::new(forward.z, 0.0, -forward.x);

        if window.is_key_down(Key::W) {
            camera.move_dir(forward.x * move_speed, 0.0, forward.z * move_speed);
        }
        if window.is_key_down(Key::S) {
            camera.move_dir(-forward.x * move_speed, 0.0, -forward.z * move_speed);
        }
        if window.is_key_down(Key::A) {
            camera.move_dir(-right.x * move_speed, 0.0, -right.z * move_speed);
        }
        if window.is_key_down(Key::D) {
            camera.move_dir(right.x * move_speed, 0.0, right.z * move_speed);
        }
        if window.is_key_down(Key::Q) {
            camera.move_dir(0.0, -move_speed, 0.0);
        }
        if window.is_key_down(Key::E) {
            camera.move_dir(0.0, move_speed, 0.0);
        }



        buffer.fill(0x202020);

        if let Some((mx, my)) = window.get_mouse_pos(minifb::MouseMode::Discard) {
            let dx = mx - last_mouse.0;
            let dy = my - last_mouse.1;
            last_mouse = (mx, my);

            camera.yaw += (dx * mouse_sensitivity) as f64;
            camera.pitch += (dy * mouse_sensitivity) as f64;

            // Clamp pitch to avoid flipping
            if camera.pitch > std::f64::consts::FRAC_PI_2 - 0.01 {
                camera.pitch = std::f64::consts::FRAC_PI_2 - 0.01;
            }
            if camera.pitch < -std::f64::consts::FRAC_PI_2 + 0.01 {
                camera.pitch = -std::f64::consts::FRAC_PI_2 + 0.01;
            }
        }



        for y in 0..height {
            for x in 0..width {
                // if (!flag)
                // {
                //     if (camera.fov == 175.0)
                //     {
                //         flag = true;
                //     }
                //     camera.fov += 0.2;
                // }
                // else if (flag)
                // {
                //     flag = !(camera.fov ==  30.0);
                //     camera.fov -= 0.2;
                //}
                let px = (2.0 * (x as f64 + 0.5) / width as f64 - 1.0)
                    * (width as f64 / height as f64)
                    * (camera.fov.to_radians() / 2.0).tan();
                let py = (1.0 - 2.0 * (y as f64 + 0.5) / height as f64)
                    * (camera.fov.to_radians() / 2.0).tan();

                // Direction in camera space
                let dir = Vertex::new(px, py, 1.0);

                // Rotate according to yaw/pitch
                let yaw = camera.yaw;
                let pitch = camera.pitch;
                let rotated_dir = Vertex::new(
                    dir.x * yaw.cos() + dir.z * yaw.sin(),
                    dir.y * pitch.cos() - dir.z * pitch.sin(),
                    -dir.x * yaw.sin() + dir.z * yaw.cos(),
                );

                let dir_len = (rotated_dir.x.powi(2) + rotated_dir.y.powi(2) + rotated_dir.z.powi(2)).sqrt();
                let ray_dir = Vertex::new(
                    rotated_dir.x / dir_len,
                    rotated_dir.y / dir_len,
                    rotated_dir.z / dir_len,
                );

                let ray = Ray::new(camera.origin.clone(), ray_dir);
                let mut nearest_t = f64::INFINITY;
                let mut pixel_color = 0x202020;

                for sphere in &spheres {
                    if let Some(hit) = sphere.intersect(&ray) {
                        if hit.distance < nearest_t {
                            nearest_t = hit.distance;

                            let brightness = dot(&hit.normal, &light_dir).max(0.0);
                            let base_color = hit.color;
                            let r = (((base_color >> 16) & 0xFF) as f64 * brightness) as u32;
                            let g = (((base_color >> 8) & 0xFF) as f64 * brightness) as u32;
                            let b = ((base_color & 0xFF) as f64 * brightness) as u32;

                            pixel_color = (r << 16) | (g << 8) | b | 0xFF000000;
                        }
                    }
                }

                buffer[y * width + x] = pixel_color;
            }
        }

        window.update_with_buffer(&buffer, width, height).unwrap();
        frames += 1;
        println!("FPS: {}", frames);
    }

}
