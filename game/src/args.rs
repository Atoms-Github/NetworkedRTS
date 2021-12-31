use std::env;

pub struct Args{
    pub launch_type: LaunchType,
    pub ip: String,
    pub player_name: Option<String>,
}
#[derive(PartialEq)]
pub enum LaunchType{
    CLIENT, SERVER
}
// E
impl Args{
    pub fn gather() -> Self{
        let mut args: Vec<String> = env::args().collect();

        args.reverse();
        let _exe_name = args.pop();
        let mut is_server = false;

        let launch_type = if let Some(value) = args.pop(){
            match value.to_lowercase().as_str(){
                "client" => {
                    LaunchType::CLIENT
                }
                "server" => {
                    LaunchType::SERVER
                }
                _ => {
                    log::warn!("First arg not server or client!");
                    LaunchType::CLIENT
                }
            }
        }else{
            log::warn!("'client'/'server' argument not specified! Using client");
            LaunchType::CLIENT
        };

        let mut ip = None;
        let mut player_name = None;
        while args.len() > 0{
            match args.pop().unwrap().as_str(){
                "+ip" => {
                    ip = args.pop();
                }
                "+name" => {
                    player_name = args.pop();
                }
                _ => {
                    break;
                }
            }
        }
        if ip.is_none(){
            ip = Some(crate::utils::get_line_input("Connection / hosting IP not specified! Enter IP:"));
        }
        if player_name.is_none() && launch_type == LaunchType::CLIENT{
            player_name = Some(crate::utils::get_line_input("Username not specified! Enter username:"));
        }

        Self{
            launch_type,
            ip: ip.unwrap(),
            player_name
        }
    }
}