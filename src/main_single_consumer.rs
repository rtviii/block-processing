use std::{time::Duration, collections::HashMap};

use actix::{Context, Actor, Handler};
// use rdkafka::{consumer, ClientConfig};
use sb_backend_3_actix::{
    actors::{
        basic::{AtrPrinter, AtrUtf8ToString, AtrStringToSerde},
        kafka::{AtrKafkaConsumer, MsgJumpToOffset},
        sb_actor::SBActor,
        sb_atr_wrapper::ActorWrap,
    },
    messages::{MsgVoid, MsgNumberedMsg, MsgRawBinary},
};
use serde_json::Value;


use crate::transaction_ops::{tx_extract_accdata, AccountProfile};

mod instruction_ops;
mod transaction_ops;

const TOPIC: &'static str = "modern_blocks_json";



pub struct AtrUtf8ParseAccounts{
    accounts_map :  HashMap<String, AccountProfile>,
}

impl SBActor for AtrUtf8ParseAccounts {
}

impl AtrUtf8ParseAccounts{
    pub fn new(accounts_map :  HashMap<String, AccountProfile>) -> Self {
        Self { accounts_map }
    }
}
impl Actor for AtrUtf8ParseAccounts { type Context = Context<Self>; }

impl Handler<MsgNumberedMsg<MsgRawBinary>> for  AtrUtf8ParseAccounts{
    type Result = ();
    fn handle(&mut self, msg: MsgNumberedMsg<MsgRawBinary>, ctx: &mut Self::Context) -> Self::Result {
        let block:Value = serde_json::from_slice(msg.msg.value.as_slice()).expect("couldnt parse block");
        

    }
}





#[actix_rt::main]
pub async fn main() {

    // let parser = AtrUtf8ParseAccounts{};

    // let actor_iowrite      = AtrIOWrite::new().wrap();
    let actor_printer      = AtrPrinter::new(None).wrap();

    // let actor_string2serde = AtrStringToSerde::new_numbered(actor_printer).wrap();
    // let actor_raw2string   = AtrUtf8ToString::new_numbered(actor_string2serde).wrap();

    let actor_raw2string   = AtrUtf8ToString::new_numbered(actor_printer).wrap();
    let consumer           = AtrKafkaConsumer::new_simple(TOPIC, actor_raw2string);
    let consumer_w         = consumer.wrap();

    consumer_w.do_send(MsgJumpToOffset { offset: 3_000_000 });

    loop {
        let mut ival = actix::clock::interval(Duration::from_millis(2000));
        ival.tick().await;
        consumer_w.do_send( MsgVoid{} );
    }
}
