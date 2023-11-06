use wasmtime::*;

pub struct WasmRuntime<T> {
    file_path: String,
    fn_name: String,
    engine: Engine,
    instance: Instance,
    pub store: Store<T>,
}

impl<T> WasmRuntime<T> {
    pub fn create(file_path: &str, fn_name: &str, value: T) -> Self {
        let engine = Engine::default();
        let mut store = Store::<T>::new(&engine, value);
        let module = Module::from_file(&engine, file_path).unwrap();
        let instance = Instance::new(&mut store, &module, &[]).unwrap();
        WasmRuntime{
            file_path: file_path.to_owned(),
            engine: engine,
            instance: instance,
            store: store,
            fn_name: fn_name.to_owned(),
        }
    }
    pub fn reload(&mut self) {
        let module = Module::from_file(&self.engine, &self.file_path).unwrap();
        let instance = Instance::new(&mut self.store, &module, &[]).unwrap();
        self.instance = instance;
    }
    pub fn exec<U: wasmtime::WasmParams, V: wasmtime::WasmResults>(&mut self) -> TypedFunc<U, V> {
        self.instance.get_typed_func::<U, V>(&mut self.store, &self.fn_name).unwrap()
    }
}

pub fn execute_wasm<T>(file_path: &str, fn_name: &str) -> T where T: WasmResults {
    let mut runtime = WasmRuntime::<i32>::create(file_path, fn_name, 0);
    let add = runtime.exec::<(), T>();
    add.call(&mut runtime.store, ()).unwrap()
}
