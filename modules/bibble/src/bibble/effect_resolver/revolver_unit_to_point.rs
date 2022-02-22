use crate::*;


impl<'a> Revolver<'a>{
    pub fn revolve_unit_to_point(&mut self, data: &GameData, effect: &EffectUnitToPoint, source: GlobalEntityID,
                                 target: &PointFloat){
        let owner = self.c.get1_unwrap::<OwnedComp>(source).owner;
        match effect{
            EffectUnitToPoint::NOTHING => {}
            EffectUnitToPoint::TO_POINT(to_point) => {
                self.revolve_to_point(data, to_point, target, owner);
            }
            _ => {
                unimplemented!()
            }
        }
    }
}

