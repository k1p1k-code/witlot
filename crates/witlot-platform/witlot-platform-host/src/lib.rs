use wasmtime::component::{Component, Linker, ResourceTable};
use wasmtime::{Config, Engine, Store};
use wasmtime_wasi::p2::add_to_linker_async;
use wasmtime_wasi::{WasiCtx, WasiCtxBuilder, WasiCtxView, WasiView};

use std::fs;
use std::path::Path;

wasmtime::component::bindgen!({
    path: "../../../wit/platform",
    world: "platform-world"
});

pub struct HostState {
    wasi_ctx: WasiCtx,
    table: ResourceTable,
}

impl WasiView for HostState {
    fn ctx(&mut self) -> WasiCtxView<'_> {
        WasiCtxView {
            ctx: &mut self.wasi_ctx,
            table: &mut self.table,
        }
    }
}


pub struct WasmPlatform{
    pub platform: PlatformWorld,
    pub store: Store<HostState>
}


pub async fn get_wasm_platforms() -> Vec<WasmPlatform>{
    let mut config = Config::new();
    config.wasm_component_model(true);
    config.wasm_component_model_async(true);

    let engine = Engine::new(&config).expect("Error create engine wasm platforms");
    let mut linker = Linker::<HostState>::new(&engine);
    add_to_linker_async::<HostState>(&mut linker).unwrap();
    let mut wasm_platforms = Vec::new();
    for entry in fs::read_dir(Path::new("platforms")).expect("Failed to read ./platforms") {
        let entry = entry.unwrap();
        let path_buf = entry.path(); 
        if !path_buf.is_dir() && path_buf.extension().and_then(|s| s.to_str()) == Some("wasm") {
            let component = Component::from_file(&engine, path_buf).unwrap();
            let wasi_ctx = WasiCtxBuilder::new()
            .inherit_stdio()
            .build();
            let state = HostState {
                wasi_ctx,
                table: ResourceTable::new(),
            };
            let mut store = Store::new(&engine, state);
            let platform = PlatformWorld::instantiate_async(&mut store, &component, &linker).await.unwrap();
            wasm_platforms.push(WasmPlatform{
                platform,
                store
            });
        }
    }

    wasm_platforms
    

}