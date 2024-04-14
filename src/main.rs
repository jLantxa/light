/*
 * light is a spectral path tracer written in Rust for educational purposes
 *
 * Copyright (C) 2024  Javier Lancha Vázquez
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

use algebra::Vec3;
use camera::{Camera, CameraConfig};
use loader::FileLoader;
use render::PathTracer;

mod algebra;
mod camera;
mod light;
mod loader;
mod material;
mod object;
mod render;
mod scene;
mod spectrum;

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let file_path = &args[1];

    let file_loader = FileLoader::new(file_path).unwrap();

    let scene = file_loader.get_scene().expect("Expected a scene");

    let camera_config = CameraConfig {
        position: Vec3::new(0.0, 0.0, 0.0),
        direction: Vec3::new(0.0, 0.0, 1.0),
        resolution: (400, 300),
        rotation: 0.0,
        fov: camera::FieldOfView::Vertical(70.0_f32.to_radians()),
        focus_mode: camera::FocusMode::PinHole,
    };
    let camera = Camera::new(&camera_config);

    // TODO: Move to json
    let wavelengths: Vec<f32> = vec![380.0e-9, 480.0e-9, 580.0e-9, 680.0e-9, 780.0e-9];

    let path_tracer = PathTracer::default();
    let image = path_tracer.render(&scene, &camera, &wavelengths);

    image.save_with_format("render.png", image::ImageFormat::Png);
}
