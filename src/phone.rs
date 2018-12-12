extern crate crossbeam_channel;

use crossbeam_channel::{Sender, Receiver};

#[derive(Debug, Clone)]
pub struct Phone<S, R> {
    sender: Sender<S>,
    receiver: Receiver<R>,
}

impl<S, R> Phone<S, R> {
    pub fn send(&self, msg: S) {
        self.sender.send(msg)
    }

    pub fn recv(&self) -> Option<R> {
        self.receiver.recv()
    }

    pub fn try_recv(&self) -> Option<R> {
        self.receiver.try_recv()
    }

    pub fn sender(&self) -> &Sender<S> {
        &self.sender
    }

    pub fn receiver(&self) -> &Receiver<R> {
        &self.receiver
    }
}

pub fn bounded<S, R>(cap: usize) -> (Phone<S, R>, Phone<R, S>) {
    let (sender1, receiver1) = crossbeam_channel::bounded(cap);
    let (sender2, receiver2) = crossbeam_channel::bounded(cap);
    (
        Phone {sender: sender1, receiver: receiver2},
        Phone {sender: sender2, receiver: receiver1},
    )
}

pub fn unbounded<S, R>() -> (Phone<S, R>, Phone<R, S>) {
    let (sender1, receiver1) = crossbeam_channel::unbounded();
    let (sender2, receiver2) = crossbeam_channel::unbounded();
    (
        Phone {sender: sender1, receiver: receiver2},
        Phone {sender: sender2, receiver: receiver1},
    )
}
