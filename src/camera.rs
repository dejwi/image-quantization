use raylib::{
    camera::Camera3D,
    consts::{KeyboardKey, MouseButton},
    drawing::RaylibDrawHandle,
    math::Vector3,
};

const CAMERA_SPEED: f32 = 50.0;
const CAMERA_UP_SPEED: f32 = 80.0;
const CAMERA_MOUSE_SENS: f32 = 0.005;

fn vector3_rotate_by_axis_angle(v: Vector3, axis: Vector3, angle: f32) -> Vector3 {
    let mut axis = axis;
    let mut angle = angle;
    // Using Euler-Rodrigues Formula
    // Ref.: https://en.wikipedia.org/w/index.php?title=Euler%E2%80%93Rodrigues_formula

    let mut result = v;

    // Vector3Normalize(axis);
    let mut length = f32::sqrt(axis.x * axis.x + axis.y * axis.y + axis.z * axis.z);
    if length == 0.0 {
        length = 1.0;
    }
    let ilength = 1.0 / length;
    axis.x *= ilength;
    axis.y *= ilength;
    axis.z *= ilength;

    angle /= 2.0;
    let mut a = f32::sin(angle);
    let b = axis.x * a;
    let c = axis.y * a;
    let d = axis.z * a;
    a = f32::cos(angle);
    let w = Vector3::new(b, c, d);

    // Vector3CrossProduct(w, v)
    let mut wv = Vector3::new(
        w.y * v.z - w.z * v.y,
        w.z * v.x - w.x * v.z,
        w.x * v.y - w.y * v.x,
    );

    // Vector3CrossProduct(w, wv)
    let mut wwv = Vector3::new(
        w.y * wv.z - w.z * wv.y,
        w.z * wv.x - w.x * wv.z,
        w.x * wv.y - w.y * wv.x,
    );

    // Vector3Scale(wv, 2*a)
    a *= 2.0;
    wv.x *= a;
    wv.y *= a;
    wv.z *= a;

    // Vector3Scale(wwv, 2)
    wwv.x *= 2.0;
    wwv.y *= 2.0;
    wwv.z *= 2.0;

    result.x += wv.x;
    result.y += wv.y;
    result.z += wv.z;

    result.x += wwv.x;
    result.y += wwv.y;
    result.z += wwv.z;

    return result;
}

fn get_camera_forward(camera: &Camera3D) -> Vector3 {
    Vector3::normalized(&(camera.target - camera.position))
}

fn get_camera_up(camera: &Camera3D) -> Vector3 {
    camera.up.normalized()
}

// Returns the cameras right vector (normalized)
fn get_camera_right(camera: &Camera3D) -> Vector3 {
    let forward = get_camera_forward(camera);
    let up = get_camera_up(camera);

    forward.cross(up)
}

fn camera_move_right(camera: &mut Camera3D, distance: f32) {
    // Scale by distance
    let right = get_camera_right(camera) * distance;

    // Move position and target
    camera.position = camera.position + right;
    camera.target = camera.target + right;
}

fn camera_move_forward(camera: &mut Camera3D, distance: f32) {
    let scaled = get_camera_forward(camera) * distance;
    camera.position += scaled;
    camera.target += scaled;
}

fn camera_pitch(camera: &mut Camera3D, angle: f32, around_target: bool) {
    // Rotation axis
    let right = get_camera_right(camera);

    // Rotate view vector around right axis
    let target_position =
        vector3_rotate_by_axis_angle(camera.target - camera.position, right, angle);

    if around_target {
        camera.position = camera.target - target_position;
    } else {
        camera.target = camera.position + target_position;
    }
}

fn camera_yaw(camera: &mut Camera3D, angle: f32, around_target: bool) {
    let target_position = vector3_rotate_by_axis_angle(
        camera.target - camera.position,
        get_camera_up(&camera),
        angle,
    );
    if around_target {
        camera.position = camera.target - target_position;
    } else {
        camera.target = camera.position + target_position;
    }
}

fn camera_move_up(camera: &mut Camera3D, distance: f32) {
    // Scale by distance
    let up = get_camera_up(camera) * distance;

    // Move position and target
    camera.position = camera.position + up;
    camera.target = camera.target + up;
}

fn camera_move_to_target(camera: &mut Camera3D, delta: f32) {
    let mut distance = camera.position.distance_to(camera.target);

    // Apply delta
    distance += delta;

    // Distance must be greater than 0
    if distance <= 0.0 {
        distance = 0.001;
    }

    // Set new distance by moving the position along the forward vector
    let forward = get_camera_forward(camera);
    camera.position = camera.target + (forward * (-distance));
}

pub fn handle_camera_controls(mut cam3d: &mut Camera3D, d: &RaylibDrawHandle<'_>) {
    let dt = d.get_frame_time();

    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
        let mouse_delta = d.get_mouse_delta();
        camera_yaw(&mut cam3d, -mouse_delta.x * CAMERA_MOUSE_SENS, false);
        camera_pitch(&mut cam3d, -mouse_delta.y * CAMERA_MOUSE_SENS, false);

        if d.is_key_down(KeyboardKey::KEY_W) {
            camera_move_forward(&mut cam3d, CAMERA_SPEED * dt);
        }
        if d.is_key_down(KeyboardKey::KEY_S) {
            camera_move_forward(&mut cam3d, -CAMERA_SPEED * dt);
        }
        if d.is_key_down(KeyboardKey::KEY_A) {
            camera_move_right(&mut cam3d, -CAMERA_SPEED * dt);
        }
        if d.is_key_down(KeyboardKey::KEY_D) {
            camera_move_right(&mut cam3d, CAMERA_SPEED * dt);
        }
        if d.is_key_down(KeyboardKey::KEY_SPACE) {
            camera_move_up(&mut cam3d, CAMERA_UP_SPEED * dt);
        }
        if d.is_key_down(KeyboardKey::KEY_LEFT_CONTROL) {
            camera_move_up(&mut cam3d, -CAMERA_UP_SPEED * dt);
        }
    }

    if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
        let mouse_delta = d.get_mouse_delta();
        camera_yaw(&mut cam3d, -mouse_delta.x * CAMERA_MOUSE_SENS, true);
        camera_pitch(&mut cam3d, -mouse_delta.y * CAMERA_MOUSE_SENS, true);
    }
    camera_move_to_target(&mut cam3d, -d.get_mouse_wheel_move());
}
