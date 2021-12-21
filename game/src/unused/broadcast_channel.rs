use crossbeam_channel::{Sender, Receiver, Select, unbounded, SendError, RecvError, TryRecvError};
use crossbeam_channel::internal::select;
use crate::netcode::netcode_types::*;
use crate::pub_types::*;
pub fn new_bc_multi<T: Clone>(rec_count: u8) -> (BcTx<T>, BcRx<T>){
    let mut txes = vec![];
    let mut rxes = vec![];
    for i in 0..rec_count{
        let (tx, rx) = unbounded();
        txes.push(tx);
        rxes.push(rx);
    }
    return (BcTx{
        txes
    }, BcRx{
        rxes
    });
}
pub fn new_bc<T: Clone>() -> (BcTx<T>, BcRx<T>){
    new_bc_multi(1)
}

#[derive(Clone)]
pub struct BcTx<T: Clone>{
    txes: Vec<Sender<T>>,
}
pub struct BcRx<T: Clone>{
    rxes: Vec<Receiver<T>>,
}
impl<T: Clone> BcTx<T>{
    pub fn send(&mut self, value: T) -> Result<(), SendError<T>>{
        for sink in &self.txes {
            let result = sink.send(value.clone()); //optimisable: 1 unnec clone.
            if result.is_err(){
                return result;
            }
        }
        return Ok(());
    }
}
impl<T: Clone> BcRx<T>{
    pub fn recv(&self) -> Result<T, RecvError>{
        self.rx().recv()
    }
    pub fn try_recv(&self) -> Result<T, TryRecvError>{
        self.rx().try_recv()
    }
    pub fn rx(&self) -> &Receiver<T>{
        return self.rxes.get(0).expect("No broadcasting rx left in me! Increase multi count by one.");
    }

    pub fn take_one_rv(&mut self) -> Self {
        let rec = self.rxes.pop().expect("No broadcasting rx left in me! Increase multi count by one.");
        return BcRx{
            rxes: vec![rec]
        };
    }
}
#[cfg(test)]
mod tests {
    use crate::netcode::common::utils::broadcast_channel::{new_bc, new_bc_multi};
    use std::thread;
    use crossbeam_channel::TryRecvError;
    use std::time::Duration;

    #[test]
    fn battlecruiser_test() {
        let (mut tx, mut rx) = new_bc_multi(2);

        assert_eq!(rx.try_recv(), Result::Err(TryRecvError::Empty));
        thread::spawn(move ||{
            tx.send(12).unwrap();
        });
        std::thread::sleep(Duration::from_millis(50));

        assert_eq!(rx.recv().unwrap(), 12);
        let split = rx.take_one_rv();

        assert_eq!(split.recv().unwrap(), 12);

        log::info!("TestsPass");
    }
}








