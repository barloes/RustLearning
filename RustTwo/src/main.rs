use actix::prelude::*;
use std::time::Duration;
use actix::{Actor, Addr, Running, StreamHandler};
use actix_web_actors::ws;
use log::info;

use actix_files::Files;
use actix_web::{web, App, Error, HttpRequest, HttpResponse, HttpServer};


#[derive(Clone, Message,Debug)]
#[rtype(result = "()")]
pub struct ChatMessage(pub String);

#[derive(Message)]
#[rtype(result = "()")]
pub struct Connect {
    addr: Recipient<ChatMessage>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct SendMessage {
    msg: String,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct AddProduct {
    product_name: u32,
    product_value: u32
}

#[derive(Default,Debug)]
struct Session;

impl Actor for Session{
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let addr = ctx.address();

        Server::from_registry()
            .send(Connect {
                addr: addr.recipient(),

            })
            .into_actor(self)
            .then(|id, act, _ctx| {


                fut::ready(())
            })
            .wait(ctx);
     }
 
     fn stopped(&mut self, ctx: &mut Self::Context) {
        println!("Actor is stopped");
     }
}

impl Handler<ChatMessage> for Session {
    type Result = ();

    fn handle(&mut self, msg: ChatMessage, ctx: &mut Self::Context) {
        println!("{:?}",msg);

        //Recipient<ChatMessages> . do_send will send through this . . .
        ctx.text(msg.0)
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for Session {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {

        println!("Handling chat messages");
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };


        match msg {
            ws::Message::Text(text) => {
                let msg = text.trim();
                if msg.starts_with('/') {
                    let mut command = text.split(' '); 
                    match command.next() {
                        Some("/add") => {
                            let ProductVec = command.collect::<Vec<&str>>();
                            println!("{}",ProductVec.len());

                            let product_name = ProductVec[0].parse::<u32>().unwrap();;
                            let product_value = ProductVec[1].parse::<u32>().unwrap();;

                            Server::from_registry()
                            .send(AddProduct { product_name: product_name, product_value: product_value })
                            .into_actor(self)
                            .then(|id, act, _ctx| {


                                fut::ready(())
                            })
                            .wait(ctx);
                        }
                        _ => ctx.text(format!("!!! unknown command: {:?}", msg)),
                    }

                    return;
                }

                //Talk to other client in the server
                Server::from_registry()
                    .send(SendMessage { msg: text })
                    .into_actor(self)
                    .then(|id, act, _ctx| {


                        fut::ready(())
                    })
                    .wait(ctx);
            }
            ws::Message::Close(reason) => {
                ctx.close(reason);
                ctx.stop();
            }
            _ => {}
        }
    }
}
#[derive(Default,Debug)]
struct Server{
    AddressList : Vec<Recipient<ChatMessage>>,
    ProductList : std::collections::HashMap::<u32, u32>,

}

impl SystemService for Server {}

impl Supervised for Server {}


impl Actor for Server{
    type Context = Context<Self>;

    fn started(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is alive");
     }
 
     fn stopped(&mut self, ctx: &mut Context<Self>) {
        println!("Actor is stopped");
     }
}

impl Handler<Connect> for Server {
    type Result = ();

    fn handle(&mut self, msg: Connect, ctx: &mut Self::Context) {
        self.AddressList.push(msg.addr);
        
        for addr in self.AddressList.iter(){
            println!("{:?}",addr);

            let content = format!(
                "Number of people in the server: {}",
                self.AddressList.len()
            );

            //this will send through the address...
            addr.do_send(ChatMessage(content));
            
        }
    }
}

impl Handler<SendMessage> for Server {
    type Result = ();

    fn handle(&mut self, msg: SendMessage, ctx: &mut Self::Context) {
        
        for addr in self.AddressList.iter(){
            let content = format!(
                "{}",
                msg.msg
            );

            //this will send through the address...
            addr.do_send(ChatMessage(content));
            
        }
    }
}

impl Handler<AddProduct> for Server{
    type Result = ();

    fn handle(&mut self, product: AddProduct, ctx: &mut Self::Context) {

        self.ProductList.insert(product.product_name, product.product_value);
        for addr in self.AddressList.iter(){
            let content = format!(
                "{:?}",
                self.ProductList
            );

            addr.do_send(ChatMessage(content));
        }
    }
}

async fn chat_route(
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    ws::start(Session::default(), &req, stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info"))
        .init();

    let addr = "127.0.0.1:8082";

    let srv = HttpServer::new(move || {
        App::new()
            .service(web::resource("/ws/").to(chat_route))
    })
    .bind(&addr)?;

    info!("Starting http server: {}", &addr);

    srv.run().await
}