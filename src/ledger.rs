use std::sync::mpsc::{Sender, Receiver, channel};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Block{

}

/* Commands */
#[derive(Serialize, Deserialize)]
pub struct GetLastBlocks{
    cnt_blocks: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetBlockByHash{
    hash: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct InsertBlock{
    block: Block,
}

#[derive(Serialize, Deserialize)]
pub enum Cmd{

}

/* End commands */

pub enum ResultData{
    Wrong(String),
}

pub struct Ledger{
    tx: Sender<Cmd>,
    rx: Receiver<Cmd>,
}

impl Ledger{
    pub fn new() -> Self{
        let (tx, rx) = channel::<Cmd>();
        Self{
            tx: tx,
            rx: rx,
        }
    }

    pub fn get_tx(&self) -> Sender<Cmd>{
        self.tx.clone()
    }

    pub fn exec_cmd(&mut self) -> ResultData{
        ResultData::Wrong("Something get wrong".to_owned())
    }
}