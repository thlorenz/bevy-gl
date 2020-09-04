pub struct CameraConfig {
    pub mov_speed: f32,
    pub mouse_sensitivity: f32,
    pub constrain_pitch: bool,
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            mov_speed: 0.01,
            mouse_sensitivity: 0.1,
            constrain_pitch: true,
        }
    }
}
