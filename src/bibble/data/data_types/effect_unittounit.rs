use super::*;

pub enum EffectUnitToUnit{
    LAUNCH_PROJECTILE(ProjectileID),
    INSTA_AFFECT_TARGET(EffectToUnit)
}


pub struct EutuLaunchProjectile{
    pub actor_id: ActorID
}