use super::*;

pub enum EffectUnitToUnit{
    LAUNCH_PROJECTILE(ProjectileID),
    INSTA_DAMAGE_TEST
}


pub struct EutuLaunchProjectile{
    pub actor_id: ActorID
}