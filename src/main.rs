use sb_backend_3_actix::{
    actors::{
        basic::{AtrLambda, AtrSyncLambda},
        kafka::{AtrKafkaConsumer, MsgJumpToOffset},
        parallelization::{AtrCollector, AtrDistributor},
        sb_atr_wrapper::ActorWrapper,
        sb_sync_actor::SBSyncActor,
    },
    messages::{MsgNumberedMsg, MsgRawBinary, MsgString},
};
use std::time::Duration;

use sb_backend_3_actix::{
    actors::{
        basic::{AtrPrinter, AtrStringToSerde, AtrUtf8ToString},
        sb_actor::SBActor,
        sb_atr_wrapper::ActorWrap,
    },
    messages::MsgVoid,
};
mod account;
// const TOPIC: &'static str = "modern_blocks_json";




#[actix_rt::main]
pub async fn main() {
    let mut consumer_w: Box<ActorWrapper<AtrKafkaConsumer>> = Box::new(ActorWrapper::default());
    let printer_w                                           = AtrPrinter::default().wrap();
    let collector_wrapper                                   = AtrCollector::new(consumer_w.clone(), printer_w, 256).wrap();
    let actor_chain                                         = collector_wrapper.clone();
    let actor_chain                                         = SBSyncActor::wrap(16, move || {
        AtrSyncLambda::new_closure(
            Box::new(move |x: MsgNumberedMsg<MsgNumberedMsg<MsgRawBinary>>| {
                // let block_json: serde_json::Value = serde_json::from_slice(x.msg.msg.value.as_slice()).expect("Unable to parse UTF8-string as JSON");
                // println!("Got blockjson : {}",block_json["blockHeight"].as_u64().unwrap());

                MsgNumberedMsg {
                    key: x.key,
                    msg: MsgString {
                        key  : None,
                        value:  format!("Processed key {}",  x.key.to_string()),
                    },
                }
            }),

            actor_chain.clone(),
        )

    });

    let mut distributor = AtrDistributor::default();

    distributor.destinations.push(actor_chain.clone());
    let actor_chain = distributor.wrap();
    let actor_chain = AtrKafkaConsumer::new_simple_hosts("modern_blocks_json", actor_chain, "localhost:9092").wrap();
    actor_chain.do_send(MsgJumpToOffset { offset: 2_500_000  });

    consumer_w.set_addr(
        actor_chain
            .as_ref()
            .addr
            .read()
            .unwrap()
            .as_ref()
            .unwrap()
            .clone(),
    );

    collector_wrapper.do_send(MsgVoid);
    // collector_wrapper.do_send(MsgJumpToOffset{offset:5659597});

    // -------------------------------------------------------------------------
    // let printer          = AtrPrinter::new(None).wrap();
    // let actor_raw2string = AtrUtf8ToString::new_numbered(printer).wrap();
    // let consumer         = AtrKafkaConsumer::new_simple(TOPIC, actor_raw2string);
    // let consumer_w       = consumer.wrap();

    // consumer_w.do_send(MsgJumpToOffset { offset: 3_000_000 });
    loop {
        actix::clock::sleep(Duration::from_millis(500)).await;
    }

    // loop {
    //     let mut ival = actix::clock::interval(Duration::from_millis(2000));
    //     ival.tick().await;
    //     consumer_w.do_send( MsgVoid{} );
    // }
}
