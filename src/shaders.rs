use nalgebra_glm::{Vec3, Vec4, Mat3, mat4_to_mat3};
use crate::vertex::Vertex;
use crate::Uniforms;
use crate::fragment::Fragment;
use crate::color::Color;

pub fn vertex_shader(vertex: &Vertex, uniforms: &Uniforms) -> Vertex {
    let position = Vec4::new(
        vertex.position.x,
        vertex.position.y,
        vertex.position.z,
        1.0
    );

    let transformed = uniforms.projection_matrix * uniforms.view_matrix * uniforms.model_matrix * position;

    let w = transformed.w;
    let transformed_position = Vec4::new(
        transformed.x / w,
        transformed.y / w,
        transformed.z / w,
        1.0
    );

    let screen_position = uniforms.viewport_matrix * transformed_position;

    let model_mat3 = mat4_to_mat3(&uniforms.model_matrix);
    let normal_matrix = model_mat3.transpose().try_inverse().unwrap_or(Mat3::identity());

    let transformed_normal = normal_matrix * vertex.normal;

    Vertex {
        position: vertex.position,
        normal: vertex.normal,
        tex_coords: vertex.tex_coords,
        color: vertex.color,
        transformed_position: Vec3::new(screen_position.x, screen_position.y, screen_position.z),
        transformed_normal: transformed_normal
    }
}

pub fn fragment_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    match uniforms.planet_type {
        0 => sun_shader(fragment, uniforms),         // Sol
        1 => mercury_shader(fragment, uniforms),    // Mercurio
        2 => venus_shader(fragment, uniforms),      // Venus
        3 => earth_shader(fragment, uniforms),      // Tierra
        4 => mars_shader(fragment, uniforms),       // Marte
        5 => jupiter_shader(fragment, uniforms),    // Júpiter
        6 => saturn_shader(fragment, uniforms),     // Saturno
        7 => uranus_shader(fragment, uniforms),     // Urano
        8 => neptune_shader(fragment, uniforms),    // Neptuno
        _ => Color::new(255, 255, 255),             // Blanco por defecto
    }
}

fn sun_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(255, 223, 0) // Amarillo brillante
}

fn mercury_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(169, 169, 169) // Gris oscuro
}

fn venus_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(205, 133, 63) // Marrón anaranjado
}

fn earth_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(0, 102, 204) // Azul puro para la Tierra
}

fn mars_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let noise_value = uniforms.noise.get_noise_3d(
        fragment.vertex_position.x * 10.0,
        fragment.vertex_position.y * 10.0,
        fragment.vertex_position.z * 10.0,
    );
    if noise_value > 0.0 {
        Color::new(178, 34, 34) // Rojo intenso
    } else {
        Color::new(139, 69, 19) // Marrón rojizo
    }
}

fn jupiter_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(255, 165, 0) // Naranja
}

fn saturn_shader(fragment: &Fragment, uniforms: &Uniforms) -> Color {
    let noise_value = uniforms.noise.get_noise_3d(
        fragment.vertex_position.x * 5.0,
        fragment.vertex_position.y * 5.0,
        fragment.vertex_position.z * 5.0,
    );
    if noise_value > 0.0 {
        Color::new(210, 180, 140) // Beige claro
    } else {
        Color::new(189, 183, 107) // Amarillo grisáceo
    }
}

fn uranus_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(173, 216, 230) // Azul claro
}

fn neptune_shader(_fragment: &Fragment, _uniforms: &Uniforms) -> Color {
    Color::new(0, 0, 255) // Azul profundo
}
