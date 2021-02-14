use bevy::{input::mouse::MouseMotion, math::clamp, prelude::*};

pub struct FlyCamera {
    pub speed: f32,
    pub max_speed: f32,
    pub sensitivity: f32,
    pub friction: f32,
    pub pitch: f32,
    pub yaw: f32,
    pub velocity: Vec3,
    pub key_forward: KeyCode,
    pub key_backward: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub enabled: bool,
}
impl Default for FlyCamera {
    fn default() -> Self {
        Self {
            speed: 1.5,
            max_speed: 0.5,
            sensitivity: 50.0,
            friction: 1.0,
            pitch: 0.0,
            yaw: 0.0,
            velocity: Vec3::zero(),
            key_forward: KeyCode::W,
            key_backward: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::Space,
            key_down: KeyCode::LShift,
            enabled: true,
        }
    }
}

fn forward_vector(rotation: &Quat) -> Vec3 {
    rotation.mul_vec3(Vec3::unit_z()).normalize()
}

fn forward_walk_vector(rotation: &Quat) -> Vec3 {
    let f = forward_vector(rotation);
    let f_flattened = Vec3::new(f.x, 0.0, f.z).normalize();
    f_flattened
}

fn strafe_vector(rotation: &Quat) -> Vec3 {
    // Rotate it 90 degrees to get the strafe direction
    Quat::from_rotation_y(90.0f32.to_radians())
        .mul_vec3(forward_walk_vector(rotation))
        .normalize()
}

fn movement_axis(
    input: &Res<Input<KeyCode>>,
    plus: KeyCode,
    minus: KeyCode,
    ) -> f32 {
    let mut axis = 0.0;
    if input.pressed(plus) {
        axis += 1.0;
    }
    if input.pressed(minus) {
        axis -= 1.0;
    }
    axis
}

fn camera_movement_system(
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
    ) {
    for (mut options, mut transform) in query.iter_mut() {
        let (axis_h, axis_v, axis_float) = if options.enabled {
            (
                movement_axis(&keyboard_input, options.key_right, options.key_left),
                movement_axis(
                    &keyboard_input,
                    options.key_backward,
                    options.key_forward,
                    ),
                    movement_axis(&keyboard_input, options.key_up, options.key_down),
                    )
        } else {
            (0.0, 0.0, 0.0)
        };

        let rotation = transform.rotation;
        let accel: Vec3 = (strafe_vector(&rotation) * axis_h)
            + (forward_walk_vector(&rotation) * axis_v)
            + (Vec3::unit_y() * axis_float);
        let accel: Vec3 = if accel.length() != 0.0 {
            accel.normalize() * options.speed
        } else {
            Vec3::zero()
        };

        let friction: Vec3 = if options.velocity.length() != 0.0 {
            options.velocity.normalize() * -1.0 * options.friction
        } else {
            Vec3::zero()
        };

        options.velocity += accel * time.delta_seconds();

        // clamp within max speed
        if options.velocity.length() > options.max_speed {
            options.velocity = options.velocity.normalize() * options.max_speed;
        }

        let delta_friction = friction * time.delta_seconds();

        options.velocity = if (options.velocity + delta_friction).signum()
            != options.velocity.signum()
            {
                Vec3::zero()
            } else {
                options.velocity + delta_friction
            };

        transform.translation += options.velocity;
    }
}

#[derive(Default)]
struct State {
    mouse_motion_event_reader: EventReader<MouseMotion>,
}

fn mouse_motion_system(
    time: Res<Time>,
    mut state: ResMut<State>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mut query: Query<(&mut FlyCamera, &mut Transform)>,
    ) {
    let mut delta: Vec2 = Vec2::zero();
    for event in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
        delta += event.delta;
    }
    if delta.is_nan() {
        return;
    }

    for (mut options, mut transform) in query.iter_mut() {
        if !options.enabled {
            continue;
        }
        options.yaw -= delta.x * options.sensitivity * time.delta_seconds();
        options.pitch += delta.y * options.sensitivity * time.delta_seconds();

        options.pitch = clamp(options.pitch, -89.9, 89.9);
        // println!("pitch: {}, yaw: {}", options.pitch, options.yaw);

        let yaw_radians = options.yaw.to_radians();
        let pitch_radians = options.pitch.to_radians();

        transform.rotation = Quat::from_axis_angle(Vec3::unit_y(), yaw_radians)
            * Quat::from_axis_angle(-Vec3::unit_x(), pitch_radians);
    }
}

pub struct FlyCameraPlugin;

impl Plugin for FlyCameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app
            .init_resource::<State>()
            .add_system(camera_movement_system.system())
            .add_system(mouse_motion_system.system());
    }
}
