// Copyright 2017 ETH Zurich. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

//! Client dispatch and request completion logic.

use std::io::Error;

use futures::future::Future;
use futures::stream::Stream;
use tokio_core::reactor::Handle;

use strymon_communication::rpc::{Incoming, Outgoing, Request, RequestBuf};

use handler::CoordinatorRef;
use strymon_rpc::coordinator::*;

/// The dispatcher assigned to each incoming connection.
pub struct Dispatch {
    coord: CoordinatorRef,
    handle: Handle,
    tx: Outgoing,
}

impl Dispatch {
    /// Creates a new dispatcher instance.
    pub fn new(coord: CoordinatorRef, handle: Handle, tx: Outgoing) -> Self {
        debug!("dispatching on new incoming connection");
        Dispatch {
            coord: coord,
            handle: handle,
            tx: tx,
        }
    }

    /// Decodes a single incoming request, spawns a new completion task where necessary.
    fn dispatch<'a>(&'a mut self, req: RequestBuf<CoordinatorRPC>) -> Result<(), Error> {
        debug!("dispatching request {:?}", req.name());
        match *req.name() {
            Submission::NAME => {
                let (req, resp) = req.decode::<Submission>()?;
                let submission = self.coord
                    .submission(req)
                    .then(|res| Ok(resp.respond(res)));

                self.handle.spawn(submission);
            }
            Termination::NAME => {
                let (req, resp) = req.decode::<Termination>()?;
                let termination = self.coord
                    .termination(req)
                    .then(|res| Ok(resp.respond(res)));

                self.handle.spawn(termination);
            }
            AddWorkerGroup::NAME => {
                let (AddWorkerGroup { job, group }, resp) =
                    req.decode::<AddWorkerGroup>()?;
                let response = self.coord
                    .add_worker_group(job, group)
                    .then(|res| Ok(resp.respond(res)));
                self.handle.spawn(response);
            }
            AddExecutor::NAME => {
                let (req, resp) = req.decode::<AddExecutor>()?;
                let id = self.coord.add_executor(req, self.tx.clone());
                resp.respond(Ok(id));
            }
            Publish::NAME => {
                let (req, resp) = req.decode::<Publish>()?;
                resp.respond(self.coord.publish(req));
            }
            Unpublish::NAME => {
                let (Unpublish { token, topic }, resp) = req.decode::<Unpublish>()?;
                resp.respond(self.coord.unpublish(token, topic));
            }
            Subscribe::NAME => {
                let (req, resp) = req.decode::<Subscribe>()?;
                let subscribe = self.coord
                    .subscribe(req)
                    .then(|res| Ok(resp.respond(res)));
                self.handle.spawn(subscribe);
            }
            Unsubscribe::NAME => {
                let (Unsubscribe { token, topic }, resp) = req.decode::<Unsubscribe>()?;
                resp.respond(self.coord.unsubscribe(token, topic));
            }
            Lookup::NAME => {
                let (Lookup { name }, resp) = req.decode::<Lookup>()?;
                resp.respond(self.coord.lookup(&name));
            }
        }

        Ok(())
    }

    /// Decodes and dispatches each incoming request.
    pub fn client(mut self, rx: Incoming<CoordinatorRPC>) -> Result<(), Error> {
        let handle = self.handle.clone();
        let client = rx.for_each(move |req| self.dispatch(req))
            .map_err(|err| {
                error!("failed to dispatch client: {:?}", err);
            });

        // handle client asynchronously
        handle.spawn(client);
        Ok(())
    }
}
