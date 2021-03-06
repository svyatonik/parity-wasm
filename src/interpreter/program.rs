use std::sync::Arc;
use std::collections::HashMap;
use std::collections::hash_map::Entry;
use parking_lot::RwLock;
use elements::Module;
use interpreter::Error;
use interpreter::env::{self, env_module};
use interpreter::module::{ModuleInstance, ModuleInstanceInterface};

/// Program instance. Program is a set of instantiated modules.
pub struct ProgramInstance {
	/// Shared data reference.
	essence: Arc<ProgramInstanceEssence>,
}

/// Program instance essence.
pub struct ProgramInstanceEssence {
	/// Loaded modules.
	modules: RwLock<HashMap<String, Arc<ModuleInstanceInterface>>>,
}

impl ProgramInstance {
	/// Create new program instance.
	pub fn new() -> Result<Self, Error> {
		Ok(ProgramInstance {
			essence: Arc::new(ProgramInstanceEssence::new()?),
		})
	}

	/// Create new program instance with predefined user-defined functions
	pub fn with_functions(funcs: env::UserFunctions) -> Result<Self, Error> {
		Ok(ProgramInstance {
			essence: Arc::new(ProgramInstanceEssence::with_functions(funcs)?),
		})
	}

	/// Instantiate module.
	pub fn add_module(&self, name: &str, module: Module) -> Result<Arc<ModuleInstance>, Error> {
		let module_instance = Arc::new(ModuleInstance::new(Arc::downgrade(&self.essence), module)?);
		let mut modules = self.essence.modules.write();
		match modules.entry(name.into()) {
			Entry::Occupied(_) => Err(Error::Program(format!("module {} already instantiated", name))),
			Entry::Vacant(entry) => {
				entry.insert(module_instance.clone());
				Ok(module_instance)
			},
		}
	}

	/// Get one of the modules by name
	pub fn module(&self, name: &str) -> Option<Arc<ModuleInstanceInterface>> {
		self.essence.module(name)
	}
}

impl ProgramInstanceEssence {
	/// Create new program essence.
	pub fn new() -> Result<Self, Error> {
		ProgramInstanceEssence::with_functions(HashMap::with_capacity(0))
	}

	/// Create new program essence with provided user-defined functions
	pub fn with_functions(funcs: env::UserFunctions) -> Result<Self, Error> {
		let mut modules = HashMap::new();
		let env_module: Arc<ModuleInstanceInterface> = Arc::new(env_module(funcs)?);
		modules.insert("env".into(), env_module);
		Ok(ProgramInstanceEssence {
			modules: RwLock::new(modules),
		})
	}

	/// Get module reference.
	pub fn module(&self, name: &str) -> Option<Arc<ModuleInstanceInterface>> {
		self.modules.read().get(name).cloned()
	}
}
