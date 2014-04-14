#![crate_id = "busybee#0.1"]

#![crate_type = "lib"]

#![feature(globs)]
#![feature(macro_rules)]
#![feature(link_args)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]

extern crate serialize;

// std
use std::cast::transmute;
use std::c_str::CString;
use std::io::{MemWriter, MemReader, IoError};
use std::io::net::ip::SocketAddr;
use std::ptr::{null};
use std::libc::{c_void, c_int, c_schar};
use std::slice::raw::buf_as_slice;

// serialize
use serialize::{json, Encodable, Decodable};

// busybee
use busybee::*;

mod busybee;

pub enum BusybeeReturncode {
    SUCCESS,
    SHUTDOWN,
    POLLFAILED,
    DISRUPTED,
    ADDFDFAIL,
    TIMEOUT,
    EXTERNAL,
    INTERRUPTED,
}

macro_rules! to_rc(
    ($rc: expr) => (
        match $rc {
            BUSYBEE_SUCCESS => SUCCESS,
            BUSYBEE_SHUTDOWN => SHUTDOWN,
            BUSYBEE_POLLFAILED => POLLFAILED,
            BUSYBEE_DISRUPTED => DISRUPTED,
            BUSYBEE_ADDFDFAIL => ADDFDFAIL,
            BUSYBEE_TIMEOUT => TIMEOUT,
            BUSYBEE_EXTERNAL => EXTERNAL,
            BUSYBEE_INTERRUPTED => INTERRUPTED,
            _ => fail!("Shouldn't happen"),
        }
    )
)

pub struct Busybee {
    inner: *mut busybee_mta
}

pub struct BusybeeMapper {
    inner: *mut busybee_mapper
}

pub type ServerID = u64;

pub type LookupFn = fn (ServerID) -> SocketAddr;

// macro_rules! println(
//     ($s: expr) => (
//         std::io::stdio::stdout().write_line($s)
//     )
// )

impl BusybeeMapper {

    pub fn new(mut lookup: LookupFn) -> BusybeeMapper {
        extern "C" fn c_lookup(user_data: *mut c_void, sid: uint64_t,
           ip_addr: *mut *c_schar, port: *mut uint16_t) -> c_int {
            let lookup: LookupFn = unsafe { transmute(user_data) };
            let addr = lookup(sid);
            unsafe {
                *ip_addr = addr.ip.to_str().to_c_str().unwrap();
                *port = addr.port;
            }
            1i32
        }

        let mapper = unsafe {
            busybee_mapper_create(transmute(lookup), Some(c_lookup))
        };

        BusybeeMapper { inner: mapper }
    }

}

macro_rules! wrap(
    ($func: ident) => (
        pub fn $func(&mut self) {
            unsafe {
                busybee_mta_$func(self.inner)
            }
        }  
    );
)

impl Busybee {

    pub fn new(server_id: ServerID, addr: SocketAddr, num_threads: uint, mapper: BusybeeMapper) -> Busybee {
        unsafe {
            let c_addr = addr.ip.to_str().to_c_str().unwrap();
            let c_port = addr.port;
            let bb = busybee_mta_create(mapper.inner, c_addr, c_port, server_id, num_threads as u64);
            Busybee { inner: bb }
        }
    }

    pub fn delete(~self) {
        unsafe {
            busybee_mta_delete(self.inner)
        }
    }

    pub fn shutdown(&mut self) {
        unsafe {
            busybee_mta_shutdown(self.inner)
        }
    }

    pub fn pause(&mut self) {
        unsafe {
            busybee_mta_pause(self.inner)
        }
    }

    pub fn unpause(&mut self) {
        unsafe {
            busybee_mta_unpause(self.inner)
        }
    }

    pub fn wake_one(&mut self) {
        unsafe {
            busybee_mta_wake_one(self.inner)
        }
    }

    pub fn set_id(&mut self, server_id: ServerID) {
        unsafe {
            busybee_mta_set_id(self.inner, server_id)
        }
    }

    pub fn set_timeout(&mut self, timeout: uint) {
        unsafe {
            busybee_mta_set_timeout(self.inner, timeout as c_int)
        }
    }

    pub fn set_ignore_signals(&mut self) {
        unsafe {
            busybee_mta_set_ignore_signals(self.inner)
        }
    }

    pub fn unset_ignore_signals(&mut self) {
        unsafe {
            busybee_mta_unset_ignore_signals(self.inner)
        }
    }

    pub fn add_signals(&mut self) {
        unsafe {
            busybee_mta_add_signals(self.inner)
        }
    }

    pub fn get_addr(&mut self, server_id: ServerID) -> Result<SocketAddr, BusybeeReturncode> {
        let mut c_ip_addr = null();
        let mut c_port = 0;
        unsafe {
            let rc = busybee_mta_get_addr(self.inner, server_id, &mut c_ip_addr, &mut c_port);
            if rc != BUSYBEE_SUCCESS {
                Err(to_rc!(rc))
            } else {
                let ip_addr = CString::new(c_ip_addr, false);
                Ok(SocketAddr {
                    ip: from_str(ip_addr.as_str().unwrap()).unwrap(),
                    port: c_port
                })
            }
        }
    }

    pub fn deliver<'a>(&'a mut self, server_id: ServerID, msg: &'a [u8]) -> bool {
        let c_str = msg.to_c_str();
        let len = c_str.len() as u64;
        unsafe {
            match busybee_mta_deliver(self.inner, server_id, c_str.unwrap(), len) {
                0 => false,
                _ => true,
            }
        }
    }

    pub fn poll_fd(&mut self) -> int {
        unsafe { busybee_mta_poll_fd(self.inner) as int }
    }

    pub fn drop(&mut self, server_id: ServerID) -> BusybeeReturncode {
        unsafe { to_rc!(busybee_mta_drop(self.inner, server_id)) }
    }

    pub fn send<'a>(&mut self, server_id: ServerID, msg: &'a [u8]) -> BusybeeReturncode {
        let c_str = msg.to_c_str();
        let len = c_str.len() as u64;
        unsafe {
            to_rc!(busybee_mta_send(self.inner, server_id, c_str.unwrap(), len))
        }
    }

    pub fn send_object<'a, T: Encodable<json::Encoder<'a>, IoError>>(&mut self, server_id: ServerID, msg: T) -> BusybeeReturncode {
        let mut w = MemWriter::new();
        let mut encoder = json::Encoder::new(&mut w as &mut std::io::Writer);
        match msg.encode(&mut encoder) {
            Ok(()) => self.send(server_id, w.unwrap()),
            Err(e) => fail!("json encoding error: {}", e),
        }
    }

    pub fn recv<'a>(&mut self) -> Result<(ServerID, ~[u8]), BusybeeReturncode> {
        let mut c_server_id = 0u64;
        let mut c_msg = null();
        let mut c_msg_sz = 0;

        unsafe {
            let rc = busybee_mta_recv(self.inner, &mut c_server_id,
                &mut c_msg, &mut c_msg_sz);
            if rc != BUSYBEE_SUCCESS {
                Err(to_rc!(rc))
            } else {
                buf_as_slice(c_msg, c_msg_sz as uint, |bytes| {
                    let bytes: &[u8] = transmute(bytes);
                    Ok((c_server_id, bytes.to_owned()))
                })
            }
        }
    }

    pub fn recv_object<T: Decodable<json::Decoder, json::Error>>(&mut self) -> Result<(ServerID, T), BusybeeReturncode> {
        let (sid, bytes) = try!(self.recv());
        println!("{}", bytes);
        println!("{}", std::str::from_utf8(bytes).unwrap());
        let mut reader = MemReader::new(bytes);
        let json_object = match json::from_reader(&mut reader as &mut std::io::Reader) {
            Ok(obj) => obj,
            Err(e) => fail!("error decoding json from bytes: {}", e),
        };
        let mut decoder = json::Decoder::new(json_object);
        match Decodable::decode(&mut decoder) {
            Ok(msg) => Ok((sid, msg)),
            Err(e) => fail!("error decoding object from json: {}", e),
        }
    }

}