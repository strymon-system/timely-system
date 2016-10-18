use std::io::{Result, Error};
use std::net::{TcpListener, TcpStream, Shutdown, ToSocketAddrs};
use std::sync::mpsc;
use std::thread;

use futures::{Future, Poll};
use futures::stream::{self, Stream};

use network::message::buf::{MessageBuf, read, write};

pub mod message;
pub mod reqresp;

pub struct Network {
    external: String,
}

impl Network {
    pub fn init<T: Into<Option<String>>>(external: T) -> Result<Self> {
        let external = external.into()
            .unwrap_or_else(|| String::from("localhost"));
        Ok(Network { external: external })
    }

    pub fn connect<E: ToSocketAddrs>(&self, endpoint: E) -> Result<(Sender, Receiver)> {
        channel(TcpStream::connect(endpoint)?)
    }

    pub fn listen<P: Into<Option<u16>>>(&self, port: P) -> Result<Listener> {
        Listener::new(self.external.clone(), port.into().unwrap_or(0))
    }
}

#[derive(Clone)]
pub struct Sender {
    tx: mpsc::Sender<MessageBuf>,
}

impl Sender {
    pub fn send<T: Into<MessageBuf>>(&self, msg: T) {
        drop(self.tx.send(msg.into()));
    }
}

pub struct Receiver {
    rx: stream::Receiver<MessageBuf, Error>,
}

impl Stream for Receiver {
    type Item = MessageBuf;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<MessageBuf>, Error> {
        self.rx.poll()
    }
}

pub struct Listener {
    external: String,
    port: u16,
    rx: stream::Receiver<(Sender, Receiver), Error>,
}

impl Listener {
    fn new(external: String, port: u16) -> Result<Self> {
        let sockaddr = ("::", port);
        let listener = TcpListener::bind(&sockaddr)?;
        let port = listener.local_addr()?.port();

        let (tx, rx) = stream::channel();
        thread::spawn(move || {
            let mut tx = tx;
            let mut is_ok = true;
            while is_ok {
                let stream = listener.accept();
                is_ok = stream.is_ok();
                let pair = stream.and_then(|(s, _)| channel(s));
                tx = match tx.send(pair).wait() {
                    Ok(tx) => tx,
                    Err(_) => break,
                }
            }
        });

        Ok(Listener {
            external: external,
            port: port,
            rx: rx,
        })
    }

    pub fn external_addr(&self) -> (&str, u16) {
        (&*self.external, self.port)
    }
}

impl Stream for Listener {
    type Item = (Sender, Receiver);
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<(Sender, Receiver)>, Error> {
        self.rx.poll()
    }
}

fn channel(stream: TcpStream) -> Result<(Sender, Receiver)> {
    let mut instream = stream.try_clone()?;
    let mut outstream = stream;

    let (sender_tx, sender_rx) = mpsc::channel();
    thread::spawn(move || {
        while let Ok(msg) = sender_rx.recv() {
            if let Err(err) = write(&mut outstream, &msg) {
                info!("unexpected error while writing bytes: {:?}", err);
                break;
            }
        }

        drop(outstream.shutdown(Shutdown::Both));
    });

    let (receiver_tx, receiver_rx) = stream::channel();
    thread::spawn(move || {
        let mut tx = receiver_tx;
        let mut is_ok = true;
        while is_ok {
            let message = read(&mut instream);
            is_ok = message.is_ok();
            tx = match tx.send(message).wait() {
                Ok(tx) => tx,
                Err(_) => break,
            }
        }

        drop(instream.shutdown(Shutdown::Both));
    });

    Ok((Sender { tx: sender_tx }, Receiver { rx: receiver_rx }))
}

fn _assert() {
    fn _is_send<T: Send>() {}
    _is_send::<Sender>();
    _is_send::<Receiver>();
    _is_send::<Network>();
}

#[cfg(test)]
mod tests {

    use futures::stream::Stream;
    use network::message::MessageBuf;
    use network::message::abomonate::Abomonate;
    use network::*;
    use std::io::Result;

    fn assert_io<F: FnOnce() -> Result<()>>(f: F) {
        f().expect("I/O test failed")
    }

    #[test]
    fn network_integration() {
        assert_io(|| {
            let network = Network::init(None)?;
            let listener = network.listen(None)?;
            let (tx, rx) = network.connect(listener.external_addr())?;

            let mut ping = MessageBuf::empty();
            ping.push::<Abomonate, _>(&String::from("Ping")).unwrap();
            tx.send(ping);

            // process one single client
            listener.and_then(|(tx, rx)| {
                    let mut ping = rx.wait().next().unwrap()?;
                    assert_eq!("Ping", ping.pop::<Abomonate, String>().unwrap());

                    let mut pong = MessageBuf::empty();
                    pong.push::<Abomonate, _>(&String::from("Pong")).unwrap();
                    tx.send(pong);
                    Ok(())
                })
                .wait()
                .next()
                .unwrap()?;

            let mut pong = rx.wait().next().unwrap()?;
            assert_eq!("Pong", pong.pop::<Abomonate, String>().unwrap());

            Ok(())
        });
    }
}
