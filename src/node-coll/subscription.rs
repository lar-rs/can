use std::collections::HashMap;
use std::sync::{atomic, Arc, RwLock};
use jsonrpc_core::futures::future::{Future};
use jsonrpc_core::{Error, ErrorCode, Result};
use jsonrpc_derive::rpc;
use jsonrpc_pubsub::typed;
use jsonrpc_pubsub::{Session, SubscriptionId};
use lazy_static::lazy_static;
// use jsonrpc_core_client::transports::local;
// use serde_derive::{Deserialize, Serialize};
lazy_static! {
    // static ref MODS: Mutex<HashMap<u64, MioPin>> = Mutex::new(HashMap::new());
}
#[derive(Default)]
pub struct Subscription {
	uid: atomic::AtomicUsize,
	active: Arc<RwLock<HashMap<SubscriptionId, typed::Sink<String>>>>,
}

impl Subscription {
    pub fn active_subscription(&self) -> Arc<RwLock<HashMap<SubscriptionId, typed::Sink<String>>>> {
        return self.active.clone()
    }
}

#[rpc]
pub trait Subscribe {
	type Metadata;
	/// Hello subscription
	#[pubsub(subscription = "hello", subscribe, name = "hello_subscribe", alias("hello_sub"))]
	fn subscribe(&self, meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: u64);

	/// Unsubscribe from hello subscription.
	#[pubsub(subscription = "hello", unsubscribe, name = "hello_unsubscribe")]
	fn unsubscribe(&self, meta: Option<Self::Metadata>, subscription: SubscriptionId) -> Result<bool>;
}


impl Subscribe for Subscription {
	type Metadata = Arc<Session>;
	fn subscribe(&self, _meta: Self::Metadata, subscriber: typed::Subscriber<String>, param: u64) {
		if param != 10 {
			subscriber
				.reject(Error {
					code: ErrorCode::InvalidParams,
					message: "Rejecting subscription - invalid parameters provided.".into(),
					data: None,
				})
				.unwrap();
			return;
		}
		let id = self.uid.fetch_add(1, atomic::Ordering::SeqCst);
		let sub_id = SubscriptionId::Number(id as u64);
		let sink = subscriber.assign_id(sub_id.clone()).unwrap();
		self.active.write().unwrap().insert(sub_id, sink);
	}

	fn unsubscribe(&self, _meta: Option<Self::Metadata>, id: SubscriptionId) -> Result<bool> {
		let removed = self.active.write().unwrap().remove(&id);
		if removed.is_some() {
			Ok(true)
		} else {
			Err(Error {
				code: ErrorCode::InvalidParams,
				message: "Invalid subscription.".into(),
				data: None,
			})
		}
	}
}
