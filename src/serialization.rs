use bytes::{Buf, BufMut};
use crate::packet::Packet;

type Error = &'static str;

pub fn serialize<T: BufMut>(buf: &mut T, packet: &Packet) -> Result<(), Error> {
    unimplemented!()
}

pub fn deserialize<T: Buf>(buf: &T) -> Result<Packet, Error>{
    unimplemented!()
}