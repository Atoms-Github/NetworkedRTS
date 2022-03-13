use std::ops::Div;

use ggez::event::MouseButton;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;


pub static PLAYER_DISCONNECT: System = System{
    run,
    name: "player_disconnect"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for player_id in meta.sim_info.get_disconnecting_players(){
        if meta.meta.quality == SimQuality::DETERMA{
            println!("Disconnecting player {}", player_id);
        }
        c.get_mut::<PlayerComp>(player_id as GlobalEntityID).unwrap().connected = false;

        if let Some(zero) = c.get_mut::<CommonOverseer>(0){
            zero.connected_players -= 1;
        }
    }
}

