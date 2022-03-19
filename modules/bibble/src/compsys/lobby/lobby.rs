use crate::*;
use std::ops::Mul;
use std::ops::Div;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LobbyManager{ // Means keep when scene changes.
    pub game_start_cooldown: f32,
}

pub static LOBBY_SYS: System = System{
    run,
    name: "lobby"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.find_scene();
    for (lobby_id, lobby) in CompIter1::<LobbyManager>::new(c){
        lobby.game_start_cooldown -= meta.meta.delta;
        // Check for game start on F1.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(VirtualKeyCode::F1){
                scene.next = RtsSceneType::InGame;
            }
        }
    }

}






