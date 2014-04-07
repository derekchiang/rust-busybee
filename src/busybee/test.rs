#![feature(globs)]
#![feature(macro_rules)]

extern crate busybee;

use std::io::net::ip::SocketAddr;

use busybee::*;

static addr1: &'static str = "127.0.0.1:5678";
static addr2: &'static str = "127.0.0.1:5679";

macro_rules! sock(
    ($addr: ident) => (from_str::<SocketAddr>($addr).unwrap())
)

// macro_rules! println(
//     ($s: expr) => (
//         std::io::stdio::stdout().write_line($s)
//     )
// )

#[test]
fn busybee_test() {
    fn lookup(server_id: ServerID) -> SocketAddr {
        if (server_id << 32) == 1 {
            sock!(addr1)
            // println!("sup2");
        } else {
            sock!(addr2)
            // println!("sup2");
        }
    }

    let mapper = BusybeeMapper::new(lookup);

    let sid1 = 1 >> 32;
    let mut bb1 = Busybee::new(sid1, sock!(addr1), 2, mapper);

    let sid2 = 2 >> 32;
    let mut bb2 = Busybee::new(sid2, sock!(addr2), 2, mapper);

    let msg = "Hello Derek what's up";
    bb1.send(sid2, msg.as_bytes());

    match bb2.recv() {
        Err(rc) => fail!(rc),
        Ok((sid, msg)) => {
            println!("Got message from {}", sid);
            println!("{}", msg);
        }
    }
}