use crate::netcode::common::sim_data::confirmed_data::ConfirmedData;

pub struct ClientDataStore{
    pub seg_data_storage: ConfirmedData,
}
impl ClientDataStore{
    pub fn new() -> Self{
        Self{

        }
    }
}