struct HitObject {
    position: [f32; 2],
    start_time: f32,
    obj_type: ObjectType,
}

enum ObjectType {
    Circle,
    Slider,
    Spinner { duration: i32 },
}
