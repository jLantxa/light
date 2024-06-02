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
use shape::{Plane, Sphere, Triangle};

fn main() {
    println!("light!");

    let mut scene = Scene::new();
    scene
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(-30.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(255.0, 0.0, 0.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(-0.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(0.0, 255.0, 0.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(30.0, 10.0, 50.0), 10.0)),
            material: Material {
                color: Color::new(0.0, 0.0, 255.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Triangle::new(
                glm::DVec3::new(-20.0, 0.0, 15.0),
                glm::DVec3::new(-10.0, 0.0, 20.0),
                glm::DVec3::new(-15.0, 5.0, 15.0),
            )),
            material: Material {
                color: Color::new(255.0, 255.0, 0.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Triangle::new(
                glm::DVec3::new(20.0, 0.0, 15.0),
                glm::DVec3::new(10.0, 0.0, 20.0),
                glm::DVec3::new(15.0, 5.0, 15.0),
            )),
            material: Material {
                color: Color::new(255.0, 0.0, 255.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Triangle::new(
                glm::DVec3::new(-5.0, 0.0, 20.0),
                glm::DVec3::new(5.0, 0.0, 20.0),
                glm::DVec3::new(0.0, 5.0, 20.0),
            )),
            material: Material {
                color: Color::new(0.0, 255.0, 255.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(-10.0, 2.0, 8.0), 2.0)),
            material: Material {
                color: Color::new(0.0, 255.0, 185.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(0.0, 2.0, 10.0), 2.0)),
            material: Material {
                color: Color::new(255.0, 185.0, 0.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(10.0, 2.0, 8.0), 2.0)),
            material: Material {
                color: Color::new(185.0, 255.0, 0.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Plane {
                position: glm::DVec3::zeros(),
                normal: glm::DVec3::y(),
            }),
            material: Material {
                color: Color::new(128.0, 128.0, 128.0),
                ..Default::default()
            },
        })
        .add_object(Object {
            shape: Box::new(Sphere::new(glm::DVec3::new(0.0, 100.0, 0.0), 1.0)),
            material: Material {
                color: Color::new(255.0, 255.0, 255.0),
                emittance: 1.0,
            },
        });

    let aperture_camera = Camera::new(&CameraConfig {
        position: glm::DVec3::new(0.0, 10.0, 00.0),
        direction: glm::DVec3::new(0.0, -10.0, 50.0),
        resolution: (800, 600),
        rotation: 0.0_f64,
        fov: FieldOfView::Horizontal(100.0_f64.to_radians()),
        focus_mode: FocusMode::FocalPlane {
            focal_distance: 50.0,
            aperture: 0.3,
        },
    });

    let pinhole_camera = Camera::new(&CameraConfig {
        position: glm::DVec3::new(0.0, 10.0, 0.0),
        direction: glm::DVec3::new(0.0, -10.0, 50.0),
        resolution: (800, 600),
        rotation: 0.0_f64,
        fov: FieldOfView::Horizontal(100.0_f64.to_radians()),
        focus_mode: FocusMode::PinHole,
    });

    let mut renderer = PathTracer::new();
    renderer.samples_per_pixel(32).max_depth(5);

    let render_image = renderer.render(&scene, &aperture_camera);
    let geo_image = render::render_geometry(&scene, &pinhole_camera);
    geo_image
        .save_with_format("target/output_geo.png", image::ImageFormat::Png)
        .expect("Expected to save file");
    render_image
        .save_with_format("target/output.png", image::ImageFormat::Png)
        .expect("Expected to save file");
}
