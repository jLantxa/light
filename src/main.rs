/*
 * light is a path tracer written in Rust for educational purposes
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

mod algebra;
mod camera;
mod color;
mod light;
mod material;
mod object;
mod render;
mod scene;
mod shape;

use camera::{Camera, CameraConfig, FieldOfView, FocusMode};
use color::Color;
use material::Material;
use object::Object;
use render::PathTracer;
use scene::Scene;
use shape::Sphere;

fn main() {
    println!("light!");

    let mut scene = Scene::new();
    scene
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(-30.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(255.0, 0.0, 0.0),
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(-0.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(0.0, 255.0, 0.0),
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(30.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(0.0, 0.0, 255.0),
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(0.0, -100000.0, 0.0), 100000.0)),
            material: Material {
                color: Color::new(128.0, 128.0, 128.0),
            },
        });

    let camera = Camera::new(&CameraConfig {
        position: glm::DVec3::new(0.0, 10.0, 0.0),
        direction: glm::DVec3::new(0.0, -10.0, 50.0),
        resolution: (400, 300),
        rotation: 0.0_f64,
        fov: FieldOfView::Horizontal(90f64.to_radians()),
        // focus_mode: FocusMode::FocalPlane {
        //     focal_distance: 5.0,
        //     aperture: 0.1,
        // },
        focus_mode: FocusMode::PinHole,
    });

    let mut renderer = PathTracer::new();
    renderer.samples_per_pixel(1).max_depth(5);

    let image = renderer.render(&scene, &camera);
    let _ = image.save_with_format("output.png", image::ImageFormat::Png);
}
