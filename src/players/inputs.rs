

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct InputState {
    pub mouse_loc: nalgebra::Point2<f32>,
    pub keys_pressed: [bool; 260],

}

impl InputState{
    pub fn new() -> InputState{
        InputState{
            mouse_loc: (),
            keys_pressed: []
        }
    }
}