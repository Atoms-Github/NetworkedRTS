use crate::ecs::comp_store::*;
use crate::utils::{TypeIdNum, gett};
use crate::rts::game::game_state::UsingResources;
use std::marker::PhantomData;
use crate::ecs::GlobalEntityID;
use std::slice::Iter;
use crate::ecs::superb_ecs::SuperbEcs;



// #[allow(non_snake_case)]
// pub struct CompIter3<'a, A : 'static, B : 'static, C : 'static> {
//     A: PhantomData<A>,
//     B: PhantomData<B>,
//     C: PhantomData<C>,
//     ecs: &'a CompStorage,
//     vec: Vec<GlobalEntityID>
// }// C:/Users/tomul/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:66
// impl<'a, A : 'static, B : 'static, C : 'static> CompIter3<'a, A, B, C>{
//     pub fn new(ecs: &'a CompStorage) -> Self{
//         let mut my_vec = ecs.query(vec![gett::<A>(), gett::<B>(), gett::<C>()]).iter().as_slice().to_vec();
//         my_vec.reverse();
//         Self{
//             A: Default::default(),
//             B: Default::default(),
//             C: Default::default(),
//             ecs,
//             vec: my_vec,
//         }
//     }
// }
// impl<'a, A, B, C> Iterator for CompIter3<'a, A, B, C>{
//     type Item = (GlobalEntityID, &'a mut A, &'a mut B, &'a mut C);
//     fn next(&mut self) -> Option<Self::Item> {
//         let entity_id = self.vec.pop()?;
//
//         return Some((entity_id,
//                      self.ecs.get_mut::<A>(entity_id).unwrap(),
//                      self.ecs.get_mut::<B>(entity_id).unwrap(),
//                      self.ecs.get_mut::<C>(entity_id).unwrap()
//         )
//         );
//     }
// }

// create_system!( render_system | secret_render_system
// 	| my_position: PositionComp, my_render: RenderComp, my_size: SizeComp, my_wasdmover_comp: WasdMoverComp
// 	|
// 	| player_names: &BTreeMap<PlayerID, String>, ctx:&mut Context
// );


#[macro_export] // Can remove.
macro_rules! comp_iter_def {
	($query_name:ident, $get_name:ident, $get_name_unwrap:ident, $($type_name:ident),+) => {
        #[allow(non_snake_case)]
        pub struct $query_name<'a, $($type_name: 'static,)+> {
            $($type_name: PhantomData<$type_name>,)+
            ecs: &'a CompStorage,
            vec: Vec<GlobalEntityID>
        }// C:/Users/tomul/.rustup/toolchains/nightly-x86_64-pc-windows-gnu/lib/rustlib/src/rust/library/core/src/slice/iter.rs:66
        impl<'a, $($type_name: 'static,)+> $query_name<'a, $($type_name,)+>{
            pub fn new(ecs: &'a CompStorage) -> Self{
                let mut my_vec = ecs.query(vec![$(gett::<$type_name>()),+]).iter().as_slice().to_vec();
                my_vec.reverse();
                Self{
                    $($type_name: Default::default(),)+
                    ecs,
                    vec: my_vec,
                }
            }
        }
        impl<'a, $($type_name: 'static,)+> Iterator for $query_name<'a, $($type_name,)+>{
            type Item = (GlobalEntityID, $(&'a mut $type_name),+);
            fn next(&mut self) -> Option<Self::Item> {
                let entity_id = self.vec.pop()?;

                return Some((entity_id,
                            $(self.ecs.get_mut::<$type_name>(entity_id).unwrap()),+
                )
                );
            }
        }
        #[allow(unused_parens)]
        impl CompStorage{
            pub fn $get_name<$($type_name : 'static),+>(&self, entity_id: GlobalEntityID) -> ($(Option<&mut $type_name>),+){
                return ($(self.get_mut::<$type_name>(entity_id)),+ );
            }
            pub fn $get_name_unwrap<$($type_name : 'static),+>(&self, entity_id: GlobalEntityID) -> ($(&mut $type_name),+){
                return ($(self.get_mut::<$type_name>(entity_id).unwrap()),+ );
            }
        }
        // #[allow(unused_parens)]
        // impl PendingEntity{
        //     pub fn $get_name<$($type_name : 'static),+>(&self, entity_id: GlobalEntityID) -> ($(Option<&mut $type_name>),+){
        //         return ($(self.get_mut::<$type_name>(entity_id)),+ );
        //     }
        //     pub fn $get_name_unwrap<$($type_name : 'static),+>(&self, entity_id: GlobalEntityID) -> ($(&mut $type_name),+){
        //         return ($(self.get_mut::<$type_name>(entity_id).unwrap()),+ );
        //     }
        // }
    };
}

comp_iter_def!(CompIter1, get1, get1_unwrap, A);
comp_iter_def!(CompIter2, get2, get2_unwrap, A, B);
comp_iter_def!(CompIter3, get3, get3_unwrap, A, B, C);
comp_iter_def!(CompIter4, get4, get4_unwrap, A, B, C, D);
comp_iter_def!(CompIter5, get5, get5_unwrap, A, B, C, D, E);
comp_iter_def!(CompIter6, get6, get6_unwrap, A, B, C, D, E, F);
comp_iter_def!(CompIter7, get7, get7_unwrap, A, B, C, D, E, F, G);
comp_iter_def!(CompIter8, get8, get8_unwrap, A, B, C, D, E, F, G, H);