#[macro_use] extern crate actix;
use actix::prelude::*;

#[derive(Message)]
struct Signal(usize);

#[derive(Message)]
struct Subscribe(Recipient<Signal>);

struct SignalServer {
    subscribers: Vec<Recipient<Signal>>
}

impl SignalServer {
    fn broadcast(&mut self) {
        for subscriber in self.subscribers.iter() {
            if let Err(e) = subscriber.do_send(Signal(self.subscribers.len())) {
                println!("Failed to invoke subscriber: {}", e);
            };
        }
    }
}

impl Actor for SignalServer {
    type Context = Context<Self>;
}

impl Handler<Subscribe> for SignalServer {
    type Result = ();

    fn handle(&mut self, msg: Subscribe, _: &mut Self::Context) {
        println!("Cheers ears");
        self.subscribers.push(msg.0);
        self.broadcast();
    }
}

struct SignalClient;

impl Actor for SignalClient {
    type Context = Context<Self>;
}

impl Handler<Signal> for SignalClient {
    type Result = ();
    fn handle(&mut self, msg: Signal, _: &mut Self::Context) {
        println!("ping! {} ", msg.0);
    }
}

fn main() {
    let main_system = System::new("main");
    println!("Hello, world!");

    let client1_addr = SignalClient.start();
    let client2_addr = SignalClient.start();

    let server_addr = SignalServer{
        subscribers: vec!()
    }.start();

    server_addr.do_send(Subscribe(client1_addr.recipient()));
    server_addr.do_send(Subscribe(client2_addr.recipient()));

    main_system.run();

}
