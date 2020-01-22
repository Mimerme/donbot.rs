use twitchchat::commands::*;
use twitchchat::*;

pub type ChatMsg = twitchchat::commands::PrivMsg;

//TODO: make this idiomatic by declaring a generic type somehwere
pub struct TwitchChat<'a> {
    channel : &'a str,
    on_message_listeners : Vec<Box<Fn(&ChatMsg) -> ()>>
}

impl<'a> TwitchChat<'a> {
    pub fn new(channel : &str) -> TwitchChat {
       return TwitchChat {
           //TODO: probably a shorthand here
            channel: channel,
            on_message_listeners: Vec::<Box<Fn(&ChatMsg) -> ()>>::new(),
       };
    }

    //NOTE: Not sure if idomatic
    pub fn add_message_listener(&mut self, g : Box<Fn(&ChatMsg) -> ()>){
        self.on_message_listeners.push(g);
    }

    pub fn connect(&self){
        // use an anonymous login (you should probably use your name and your chat oauth token)
        let (nick, token) = twitchchat::ANONYMOUS_LOGIN;
        
        // connect with this nick, token
        let mut client = twitchchat::connect_easy(nick, token)
            .unwrap() // this is an error if
                      // the network connection can't be opened,
                      // the nick/pass is invalid, etc
             // add some filters
            .filter::<commands::PrivMsg>() // filter to PrivMsg commands
            .filter::<commands::Join>();   // filter to Join commands
        
        // get a clonable, threadsafe writer
        let writer = client.writer();
        
        // for each event from the client (this &mut is only needed if you want to use `wait_for_close`)
        for event in &mut client {
            match event {
                // when we're connected
                Event::IrcReady(..) => {
                    // join a channel
                    writer.join(self.channel).unwrap();
                }
                // when we get a priv msg
                Event::Message(Message::PrivMsg(msg)) => {
                    // print out the sender : messsage
                    for listener in self.on_message_listeners.iter() {
                        (*listener)(&msg);
                    }
                }
                // when we get a join msg
                Event::Message(Message::Join(msg)) => {
                    // print out the user and the channel that was joined
                    println!("*** {} joined {}", msg.user(), msg.channel())
                }
                // when we get an error
                Event::Error(err) => {
                    // print it out
                    eprintln!("error: {}", err);
                    // and exit the loop
                    break;
                }
                // not used here
                Event::TwitchReady(..) => {
                    // this only happens when you're using Capability::Tags, Capability::Commands and a non-anonymous login
                }
                // make the compiler happy
                _ => ()
            }
        }

    }
}
