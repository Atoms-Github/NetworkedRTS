use crate::*;
use std::ops::Mul;
use std::ops::Div;
use ggez::event::KeyCode;


#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct LobbyManagerComp { // Means keep when scene changes.
    pub chosen_jigsaw: String,
}

pub static LOBBY_SYS: System = System{
    run,
    name: "lobby"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    let scene = c.query_single_comp::<JigsawSceneManager>().unwrap();
    for (lobby_id, lobby) in CompIter1::<LobbyManagerComp>::new(c){
        // Check for game start on F1.
        for (player_id , input, player) in CompIter2::<InputComp, PlayerComp>::new(c) {
            if input.inputs.primitive.is_keycode_pressed(KeyCode::F1) && !lobby.chosen_jigsaw.eq(""){
                scene.next = JigsawSceneType::InJigsaw;
            }
        }
    }
}