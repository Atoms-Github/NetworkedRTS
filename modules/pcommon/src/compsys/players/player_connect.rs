use std::ops::Div;

use ggez::event::MouseButton;

use netcode::common::net_game_state::StaticFrameData;

use crate::*;
use crate::archetypes::new_cursor;


pub static PLAYER_CONNECT: System = System{
    run,
    name: "player_connect"
};
fn run(c: &mut CompStorage, meta: &StaticFrameData){
    for (player_id, username, color) in meta.sim_info.get_connecting_players(){
        if meta.meta.quality == SimQuality::DETERMA{
            println!("Connecting player {}", player_id);
        }

        let player_ent_id = player_id as GlobalEntityID;
        let cursor = new_cursor(player_ent_id, color);
        c.req_create_entity(cursor);

        c.get_mut::<PlayerComp>(player_ent_id).unwrap().name = username;
        c.get_mut::<PlayerComp>(player_ent_id).unwrap().color = color;
        c.get_mut::<PlayerComp>(player_ent_id).unwrap().connected = true;

        if let Some(zero) = c.get_mut::<CommonOverseer>(0){
            zero.connected_players += 1;
        }
    }
}

