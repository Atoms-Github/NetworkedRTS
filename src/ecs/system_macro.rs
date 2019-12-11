// Yes, it uses get_position as both a function call and a variable. There wasn't any other way to do it while maintaining auto-completion.
#[allow(dead_code)]

pub struct TestWoah{
    pub field: i32
}

#[macro_export]
macro_rules! foo_test {
    () => ()
}


#[macro_export]
macro_rules! create_system {
	($system:ident | $internal_system:ident | $($var_name:ident : $sty:ty),+ | $($var_name2:ident : $sty2:ty),* | $($extra_arg_name:ident : $extra_arg_ty:ty),*) => {
        pub fn $internal_system (
                world: &World,
                pending: &mut PendingEntities,
                $( // The components you want to modify and read
                    $var_name: &mut VerticalStorage<$sty>,
                )+
                $( // The components you want to get
                    $var_name2: &mut VerticalStorage<$sty2>,
                )*
                $(
                    $extra_arg_name: $extra_arg_ty,
                )*
                ) {
                
            let mut types = BTreeSet::new();
            $(
                types.insert(unsafe { std::intrinsics::type_id::<$sty>() }); // The only difference between var_name and var_name2
            )+
            let request = Request{
                types
            };
            let composition_ids : Vec<usize> = World::internal_make_request(world.internal_get_composition_list(), request);

            let mut pending_for_entities = PendingEntities::new();
            
			let mut data = Data {
                    world,
                    pending: &mut pending_for_entities,
                    internals: InternalsData {
                        $(
                            $var_name,
                        )+
                        $(
			                $var_name2,
			            )*
                    }
                };
                
            for composition_id in composition_ids {
                let mut entity = Entity {
                    composition_id,
                    internal_index: 0,
                };
                let length = world.internal_get_composition_list().get(composition_id).unwrap().global_entity_ids.len();
                for j in 0..length {
                    entity.internal_index = j;
                    $system(&mut data, entity.clone(), $($extra_arg_name, )*);
                }
            }

            pending.merge(pending_for_entities);
        }

        paste::item! { // Just to hide the internals from the autocompleter.
        #[derive(Debug)]
        struct InternalsData<'a> {
            $(
                $var_name: &'a mut VerticalStorage<$sty>,
            )+
            $(
                $var_name2: &'a mut VerticalStorage<$sty2>,
            )*
        }
        }
        
        #[derive(Debug, Clone)]
        struct Entity {
			composition_id: CompositionID,
			internal_index: usize,
        }
        #[derive(Debug)]
        struct Data<'a> {
            world: &'a World,
            pending: &'a mut PendingEntities,
            internals: InternalsData<'a>,
        }
        
        impl <'a> Data<'a> {
            fn get_entity(&self, id: EntityID) -> Entity {
                Entity {
                    composition_id: self.world.get_entity_composition_id(id),
                    internal_index: self.world.get_entity_internal_index(id),
                }
            }
        }
        
        impl Entity {
            /// Literally just calls pending.destroy_entity(getId(data))
            #[inline]
            fn delete(&self, data: &mut Data) {
                let id = self.get_id(data);
                data.pending.destroy_entity(id);
            }
            
            #[inline]
            fn get_id(&self, data: &Data) -> EntityID {
                data.world.internal_get_composition_list().get(self.composition_id).unwrap().global_entity_ids.get(self.internal_index).unwrap().clone()
            }
            
            $(
                #[inline]
                fn $var_name<'a, 'b>(&self, data: &'b mut Data<'a>) -> &'b mut $sty {
                    data.internals.$var_name.get_mut(self.composition_id).unwrap().get_mut(self.internal_index).unwrap()
                }
            )+
        }
    };
}





























