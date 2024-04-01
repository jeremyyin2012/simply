use crate::providers::ping::PingProvider;
use crate::providers::doc::DocProvider;
use crate::store::Store;

mod ping;
mod doc;

#[derive(Clone)]
pub struct Providers {
    store: Store,
}

impl Providers {
    pub fn new(store: &Store) -> Self {
        Self {
            store: store.clone(),
        }
    }

    pub fn ping(&self) -> PingProvider {
        PingProvider::new(self.store.clone())
    }

    pub fn doc(&self) -> DocProvider {
        DocProvider::new(self.store.clone())
    }
}
