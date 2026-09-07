use crate::instance::InstanceStore;
use crate::models::Environment;
use crate::platform::System;
use crate::util::AsyncCaller;
use std::future::Future;
use std::rc::Rc;

pub struct Context {
    system: Rc<dyn System>,
    env: Environment,
    instance_store: Box<dyn InstanceStore>,
    async_caller: AsyncCaller,
}

impl Context {
    pub fn new(
        system: Rc<dyn System>,
        env: Environment,
        instance_store: Box<dyn InstanceStore>,
    ) -> Self {
        Self {
            system,
            env,
            instance_store,
            async_caller: AsyncCaller::new(),
        }
    }

    pub fn call_async<F: Future>(&self, future: F) -> F::Output {
        self.async_caller.call(future)
    }

    pub fn get_system(&self) -> &dyn System {
        self.system.as_ref()
    }

    pub fn get_env(&self) -> &Environment {
        &self.env
    }

    pub fn get_instance_store(&self) -> &dyn InstanceStore {
        self.instance_store.as_ref()
    }
}
