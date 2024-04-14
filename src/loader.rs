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

use serde_json;
use std::fs::File;
use std::io::BufReader;

use crate::algebra::Vec3;
use crate::camera::Camera;
use crate::material::MaterialProperties;
use crate::object::{MaterialObject, Sphere};
use crate::scene::Scene;

pub struct FileLoader {
    path: String,
    data: serde_json::Value,
}

pub struct ParseError {
    value: String,
    msg: String,
}

impl FileLoader {
    pub fn new(path: &str) -> Result<Self, std::io::Error> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let data = serde_json::from_reader(reader)?;

        let file_loader = Self {
            path: String::from(path),
            data: data,
        };

        return Ok(file_loader);
    }

    pub fn get_scene(&self) -> Option<Scene> {
        let scene_object = self.data.get("scene");

        return match scene_object {
            Some(scene_value) => {
                let objects = self.parse_objects(&scene_value);
                match objects {
                    Ok(objects) => {
                        let mut scene = Scene::default();

                        for object in objects {
                            scene.add_object(object);
                        }

                        Some(scene)
                    }
                    _ => None,
                }
            }
            _ => None,
        };
    }

    pub fn get_cameras(&self) -> Vec<Camera> {
        return todo!();
    }

    fn parse_f32_from_array(
        &self,
        array_obj: &serde_json::Value,
        value: &serde_json::Value,
    ) -> Result<f32, ParseError> {
        if let Some(x) = value.as_f64() {
            return Ok(x as f32);
        } else {
            return Err(ParseError {
                value: array_obj.to_string(),
                msg: String::from(""),
            });
        }
    }

    fn parse_vec3(&self, array_obj: &serde_json::Value) -> Result<Vec3, ParseError> {
        let array = array_obj.as_array().expect("Object is not an array");

        let x = self.parse_f32_from_array(array_obj, &array[0])?;
        let y = self.parse_f32_from_array(array_obj, &array[1])?;
        let z = self.parse_f32_from_array(array_obj, &array[2])?;

        Ok(Vec3::new(x, y, z))
    }

    fn parse_sphere(
        &self,
        sphere_obj: &serde_json::Value,
    ) -> Result<Box<dyn MaterialObject>, ParseError> {
        let center_json = sphere_obj.get("center");
        let radius_json = sphere_obj.get("radius");

        if center_json.is_none() {
            return Err(ParseError {
                value: sphere_obj.to_string(),
                msg: String::from("Sphere object defines no center"),
            });
        }

        if radius_json.is_none() {
            return Err(ParseError {
                value: sphere_obj.to_string(),
                msg: String::from("Sphere object defines no radius"),
            });
        }

        let center = self.parse_vec3(&center_json.unwrap());
        let radius: f32 = radius_json.unwrap().as_f64().unwrap() as f32;

        Ok(Box::new(Sphere::new(
            center?,
            radius,
            MaterialProperties::default(),
        )))
    }

    fn parse_object(
        &self,
        obj_json: &serde_json::Value,
    ) -> Result<Box<dyn MaterialObject>, ParseError> {
        let obj_type = obj_json.get("type");
        if let Some(obj_type) = obj_type {
            let obj_type_str = obj_type.as_str();
            return match obj_type_str {
                Some("sphere") => Ok(self.parse_sphere(obj_json)?),
                Some(unknown_type_str) => Err(ParseError {
                    value: obj_json.to_string(),
                    msg: String::from(format!("Unknown object type {}", unknown_type_str)),
                }),
                None => Err(ParseError {
                    value: obj_json.to_string(),
                    msg: String::from("No object type found"),
                }),
            };
        } else {
            return Err(ParseError {
                value: obj_json.to_string(),
                msg: String::from("Cosa"),
            });
        }
    }

    fn parse_objects(
        &self,
        scene_value: &serde_json::Value,
    ) -> Result<Vec<Box<dyn MaterialObject>>, ParseError> {
        let objects_json = scene_value.get("objects");

        match objects_json {
            Some(objects_json) => {
                let mut objects: Vec<Box<dyn MaterialObject>> = Vec::new();

                if let Some(obj_array) = objects_json.as_array() {
                    for obj_json in obj_array {
                        let object = self.parse_object(obj_json)?;
                        objects.push(object);
                    }
                }

                return Ok(objects);
            }
            None => return Ok(Vec::new()),
        }
    }
}
