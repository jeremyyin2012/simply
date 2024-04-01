use crate::model::Context;
use crate::providers::Providers;
use crate::services::ping::PingService;
use crate::services::doc::DocService;
use crate::store::Store;

mod ping;
mod doc;

#[derive(Clone)]
pub struct Services {
    ctx: Context,
    pvd: Providers,
}

impl Services {
    pub fn new(context: Context, providers: Providers) -> Self {
        Self {
            ctx: context,
            pvd: providers,
        }
    }

    pub fn ping(&self) -> PingService {
        PingService::new(self.ctx.clone(), self.pvd.clone())
    }

    pub fn doc(&self) -> DocService {
        DocService::new(self.ctx.clone(), self.pvd.clone())
    }
}
