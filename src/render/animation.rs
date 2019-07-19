pub type AnimationId = i32;

#[derive(Debug)]
pub struct AnimationState {
    animation: AnimationId,
    current_frame: i32,
    is_playing: bool,
    is_loop: bool
}