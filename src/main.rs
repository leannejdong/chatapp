
#[macro_use] extern crate rocket;

use rocket::{form::Form,
    tokio::sync::broadcast::{channel, Sender, error::RecvError}, 
    tokio::select,
serde::{Serialize, Deserialize}, 
Shutdown, 
response::stream::{EventStream, Event},
State, fs::{FileServer,relative},
};


// #[get("/world")]
// fn world() -> &'static str{
//     "Crypto won't die!"
// }

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]

struct Message{
    #[field(validate = len(..30))]
    pub room: String,
    #[field(validate = len(..20))]
    pub username: String,
    pub message: String,
}

#[post("/message", data = "<form>")]
fn post(form: Form<Message>, queue: &State<Sender<Message>>){
    let _res = queue.send(form.into_inner());
}

#[get("/events")]
async fn events(queue: &State<Sender<Message>>, mut end: Shutdown) -> EventStream![]
{
    let mut rx = queue.subscribe();

    EventStream!{
        loop{
            let msg = select!{
                msg = rx.recv() => match msg{
                    Ok(msg)=>msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _=&mut end=> break,
            };
            yield Event::json(&msg);
        }
    }
}

#[launch]
fn rocket()->_{
    //creates a new rocket server instance
    rocket::build()
        .manage(channel::<Message>(1024).0)
        // mount our routes
        .mount("/", routes![post, events])
        // mount a handler that will serve static files
        .mount("/", FileServer::from(relative!("static")))
}
