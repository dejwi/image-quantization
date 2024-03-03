use std::collections::HashSet;
use std::ffi::{c_void, CString};
use std::slice;

use clap::Parser;
use image_quantization::camera::handle_camera_controls;
use image_quantization::{
    cluster_of_color, generate_new_means, recluster_state, update_means, ColorHash,
};
use raylib::consts::KeyboardKey;
use raylib::ffi::{ExportImage, ImageFormat, LoadImage, LoadTextureFromImage, UpdateTexture};
use raylib::prelude::*;

#[derive(Parser)]
#[command(name = "ImageColorQuantization")]
struct Args {
    #[arg(long = "image", short = 'i', default_value = "lena128.png")]
    image_path: String,

    /// Number of colors image will be quantized to
    #[arg(long = "color", short = 'k', default_value = "16")]
    color_count: usize,

    /// Size of a displayed cube
    #[arg(long, default_value = "0.25")]
    cube_size: f32,

    /// Radius of displayed cubes
    #[arg(long = "radius", short = 'r', default_value = "20.0")]
    cluster_radius: f32,
}

fn main() {
    let args = Args::parse();

    let mean_size = args.cube_size * 2.0;

    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Image Color Quantization")
        .resizable()
        .build();

    let mut means = vec![Vector3::zero(); args.color_count];
    let mut clusters: Vec<Vec<Vector3>> = vec![Vec::new(); args.color_count];
    let mut samples: Vec<Vector3> = Vec::new();

    let mut show_cluster_color = true;

    let mut image = unsafe {
        let cstring_path = CString::new(&args.image_path[..]).expect("CString failed");
        LoadImage(cstring_path.as_ptr())
    };
    if image.width == 0 && image.height == 0 {
        panic!("Failed to load the image");
    }
    unsafe {
        ImageFormat(
            &mut image,
            PixelFormat::PIXELFORMAT_UNCOMPRESSED_R8G8B8A8 as i32,
        );
    }

    let image_points_count = (image.width * image.height) as usize;

    // Convert the raw pointer to a slice of Color
    let data_ptr = image.data as *mut Color;
    let points_slice = unsafe { slice::from_raw_parts_mut(data_ptr, image_points_count) };

    let unique_colors: HashSet<ColorHash> =
        HashSet::from_iter(points_slice.iter().map(|val| ColorHash(val.clone())));
    for c in unique_colors.iter() {
        samples.push(Vector3::new(
            c.r as f32 / 255.0 * args.cluster_radius,
            c.g as f32 / 255.0 * args.cluster_radius,
            c.b as f32 / 255.0 * args.cluster_radius,
        ));
    }

    // generate_new_state(&mut samples, &mut means, cluster_count, cluster_radius);
    generate_new_means(&mut means, args.cluster_radius);
    recluster_state(&mut samples, &mut means, &mut clusters);

    let rgb_center = args.cluster_radius / 2.0;
    let mut camera = Camera3D::perspective(
        Vector3::new(0.0, 60.0, 60.0),
        Vector3::new(rgb_center, rgb_center, rgb_center),
        Vector3::up(),
        90.0,
    );

    let bg = Color::from_hex("161616").expect("Invalid bg hex");

    let mut preview_image_points = Vec::from(&points_slice[..]);

    let preview_image_width = 250.0;
    let imgrec = Rectangle::new(0.0, 0.0, image.width as f32, image.height as f32);
    let destrec = Rectangle::new(
        0.0,
        0.0,
        preview_image_width,
        (image.height as f32) * (preview_image_width / (image.width as f32)),
    );

    let texture = unsafe { Box::new(LoadTextureFromImage(image)) };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(bg);
        // update clusters
        if d.is_key_pressed(KeyboardKey::KEY_P) {
            update_means(&mut means, &mut clusters, args.cluster_radius);
            recluster_state(&mut samples, &mut means, &mut clusters);

            // update image for preview
            for i in 0..image_points_count {
                let k = {
                    let k = cluster_of_color(points_slice[i], args.cluster_radius, &means);
                    if k < 0 {
                        panic!("Color out of cluster");
                    }
                    k as usize
                };

                let color = Color::new(
                    (means[k].x * 255.0 / args.cluster_radius) as u8,
                    (means[k].y * 255.0 / args.cluster_radius) as u8,
                    (means[k].z * 255.0 / args.cluster_radius) as u8,
                    255,
                );

                preview_image_points[i] = color;
            }
            // texture = unsafe { Box::new(LoadTextureFromImage(preview_image)) };
            unsafe {
                let ptr = (&preview_image_points[..]).as_ptr() as *const c_void;
                UpdateTexture(*texture, ptr);
            }
        }
        // change display color mode
        if d.is_key_pressed(KeyboardKey::KEY_O) {
            show_cluster_color = !show_cluster_color;
        }
        // display debug info
        if d.is_key_pressed(KeyboardKey::KEY_I) {
            println!("means: {}, clusters: {}", means.len(), clusters.len());
            for (i, cl) in clusters.iter().enumerate() {
                println!("Cluster{}: {}", i, cl.len());
            }
            println!("---------");
        }
        // save new image
        if d.is_key_pressed(KeyboardKey::KEY_C) {
            for i in 0..image_points_count {
                let k = {
                    let k = cluster_of_color(points_slice[i], args.cluster_radius, &means);
                    if k < 0 {
                        panic!("Color out of cluster");
                    }
                    k as usize
                };

                let color = Color::new(
                    (means[k].x * 255.0 / args.cluster_radius) as u8,
                    (means[k].y * 255.0 / args.cluster_radius) as u8,
                    (means[k].z * 255.0 / args.cluster_radius) as u8,
                    255,
                );

                points_slice[i] = color;
            }

            let st = CString::new(format!("output-{}", &args.image_path))
                .expect("Export string path failed");
            unsafe {
                ExportImage(image, st.as_ptr());
            }
        }

        d.draw_texture_pro(
            &texture,
            imgrec,
            destrec,
            Vector2::zero(),
            0.0,
            Color::WHITE,
        );

        handle_camera_controls(&mut camera, &d);
        let mut mode_3d = d.begin_mode3D(camera);

        // draw clusters
        for i in 0..args.color_count {
            let mean = means[i];
            mode_3d.draw_cube(mean, mean_size, mean_size, mean_size, Color::WHITE);

            for c in clusters[i].iter() {
                let color_pos = if show_cluster_color { &mean } else { &c };

                let color = Color::new(
                    (color_pos.x * 255.0 / args.cluster_radius) as u8,
                    (color_pos.y * 255.0 / args.cluster_radius) as u8,
                    (color_pos.z * 255.0 / args.cluster_radius) as u8,
                    255,
                );

                mode_3d.draw_cube(c, args.cube_size, args.cube_size, args.cube_size, color);
            }
        }
    }
}

// fn generate_cluser(center: Vector3, radius: f32, count: usize, samples: &mut Vec<Vector3>) {
//     let mut rng = rand::thread_rng();
//     for _ in 0..count {
//         let mag: f32 = rng.gen::<f32>() * radius;
//         let theta = rng.gen::<f32>() * 2.0 * PI;
//         let phi: f32 = rng.gen::<f32>() * 2.0 * PI;

//         let sample = Vector3::new(
//             f32::sin(theta) * f32::cos(phi) * mag,
//             f32::sin(theta) * f32::sin(phi) * mag,
//             f32::cos(theta) * mag,
//         );
//         samples.push(sample + center);
//     }
// }

// fn generate_new_state(mut samples: &mut Vec<Vector3>, count: usize, cluster_radius: f32) {
//     generate_cluser(Vector3::zero(), cluster_radius, count, &mut samples);
//     generate_cluser(
//         Vector3::new(-cluster_radius, cluster_radius, 0.0),
//         cluster_radius / 2.0,
//         count,
//         &mut samples,
//     );
//     generate_cluser(
//         Vector3::new(cluster_radius, cluster_radius, 0.0),
//         cluster_radius / 2.0,
//         count,
//         &mut samples,
//     );
// }
