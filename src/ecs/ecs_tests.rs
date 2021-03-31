


#[cfg(test)]
mod tests {
    use crate::ecs::ecs_manager::*;
    use crate::rts::systems::velocity_system::VelSystem;

    #[test]
    fn it_works() {
        let mut ecs_system = EcsManager::new();
        let mut systems = SystemsLookup::new();
        systems.add_sys(VelSystem{});

        ecs_system.sim(&systems);

    }
}

