#![feature(pin, arbitrary_self_types, futures_api)]
#![doc(html_root_url = "https://docs.rs/tokio-tcp/0.1.2")]
#![deny(missing_docs, warnings, missing_debug_implementations)]

//! UDP bindings for `tokio`.
//!
//! This module contains the UDP networking types, similar to the standard
//! library, which can be used to implement networking protocols.
//!
//! The main struct for UDP is the [`UdpSocket`], which represents a UDP socket.
//! Reading and writing to it can be done using futures, which return the
//! [`RecvDgram`] and [`SendDgram`] structs respectively.
//!
//! For convenience it's also possible to convert raw datagrams into higher-level
//! frames.
//!
//! [`UdpSocket`]: struct.UdpSocket.html
//! [`RecvDgram`]: struct.RecvDgram.html
//! [`SendDgram`]: struct.SendDgram.html
//! [`UdpFramed`]: struct.UdpFramed.html
//! [`framed`]: struct.UdpSocket.html#method.framed

mod socket;
mod send_dgram;
mod recv_dgram;

pub use self::socket::UdpSocket;
pub use self::send_dgram::SendDgram;
pub use self::recv_dgram::RecvDgram;
