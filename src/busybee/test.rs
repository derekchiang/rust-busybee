#![feature(globs)]
#![feature(macro_rules)]

extern crate busybee;
extern crate serialize;

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

#[deriving(Encodable, Decodable, Eq, Show, Clone)]
struct Message {
    body: ~str,
    id: uint,
}

#[test]
fn busybee_test() {
    fn lookup(server_id: ServerID) -> SocketAddr {
        if (server_id << 32) == 1 {
            sock!(addr1)
        } else {
            sock!(addr2)
        }
    }

    let mapper = BusybeeMapper::new(lookup);

    let sid1 = 1 << 32;
    let mut bb1 = Busybee::new(sid1, sock!(addr1), 2, mapper);

    let sid2 = 2 << 32;
    let mut bb2 = Busybee::new(sid2, sock!(addr2), 2, mapper);

    let msg = ~"Hello Derek what's up";
    bb1.send(sid2, msg.as_bytes());

    match bb2.recv() {
        Err(rc) => fail!(rc),
        Ok((sid, reply)) => {
            assert_eq!(sid, sid1);
            // The reply is NULL-terminated
            assert_eq!(reply, msg.into_bytes());
        }
    }

    let msg = Message {
        body: ~"hello",
        id: 1,
    };
    bb2.send_object(sid1, msg.clone());

    match bb1.recv_object::<Message>() {
        Err(rc) => fail!(rc),
        Ok((sid, reply)) => {
            assert_eq!(sid, sid2);
            assert_eq!(reply, msg);
        }
    }
}