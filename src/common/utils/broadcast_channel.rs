use crossbeam_channel::{Sender, Receiver, Select, unbounded, SendError, RecvError};
use crossbeam_channel::internal::select;

pub fn new_bc_multi<T: Clone>(rec_count: u8) -> (BcTx<T>, BcRx<T>){
    let (initial_tx, initial_rx) = unbounded();
    let (hooks_tx, hooks_rx) = unbounded();
    return (BcTx{
        txs: vec![initial_tx.clone()],
        new_hooks_rx: hooks_rx,
        new_hooks_tx: hooks_tx.clone()
    }, BcRx{
        rx: initial_rx,
        self_tx: initial_tx,
        new_hooks_tx: hooks_tx,
    });
}

pub fn new_bc<T: Clone>() -> (BcTx<T>, BcRx<T>){
    return new_bc_multi(1);
}

pub struct BcTx<T: Clone>{
    txs: Vec<Sender<T>>,
    new_hooks_rx: Receiver<Sender<T>>,
    new_hooks_tx: Sender<Sender<T>>,
}
pub struct BcRx<T: Clone>{
    pub rx: Receiver<T>,
    self_tx: Sender<T>,
    new_hooks_tx: Sender<Sender<T>>,
}
impl<T: Clone> BcTx<T>{
    pub fn send(&mut self, value: T) -> Result<(), SendError<T>>{
        let mut next = self.new_hooks_rx.try_recv();
        while next.is_ok(){
            self.txs.push(next.unwrap());
            next = self.new_hooks_rx.try_recv();
        }
        for sink in &self.txs {
            let result = sink.send(value.clone()); //TODO2: 1 unnec clone.
            if result.is_err(){
                return result;
            }
        }
        return Ok(());
    }
    pub fn gen_rx(&mut self) -> BcRx<T>{
        let (tx, rx) = unbounded();
        self.txs.push(tx.clone());

        return BcRx{
            rx,
            self_tx: tx,
            new_hooks_tx: self.new_hooks_tx.clone()
        };
    }
}
impl<T: Clone> BcRx<T>{
    pub fn recv(&self) -> Result<T, RecvError>{
        self.rx.recv()
    }
}
impl<T: Clone> Clone for BcRx<T>{
    fn clone(&self) -> Self {
        let (tx, rx) = unbounded();
        self.new_hooks_tx.send(tx.clone()).unwrap();
        return BcRx{
            rx,
            self_tx: tx,
            new_hooks_tx: self.new_hooks_tx.clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::common::utils::broadcast_channel::new_bc;
    use std::thread;

    #[test]
    fn it_works() {
        let (mut tx, mut rx) = new_bc();

        thread::spawn(move ||{
            tx.send(12).unwrap();
        });
        assert_eq!(rx.recv().unwrap(), 12);
        println!("TestsPass");
    }
}








