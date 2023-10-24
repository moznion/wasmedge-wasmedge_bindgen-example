use std::error::Error;
use std::path::Path;
use wasmedge_sdk::{
    config::{CommonConfigOptions, ConfigBuilder, HostRegistrationConfigOptions},
    dock::{Param, VmDock},
    Module, VmBuilder,
};

fn init_wasm_vm(wasm_filepath: &Path) -> Result<VmDock, Box<dyn Error>> {
    let module = Module::from_file(None, wasm_filepath)?;

    let config = ConfigBuilder::new(CommonConfigOptions::default())
        .with_host_registration_config(HostRegistrationConfigOptions::default().wasi(true))
        .build()?;

    if !config.wasi_enabled() {
        return Err("wasi is unable on WasmEdge configuration".into());
    }

    Ok(VmDock::new(
        VmBuilder::new()
            .with_config(config)
            .build()?
            .register_module(None, module)?,
    ))
}

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();
    let wasm_file_path = Path::new(&args[1]);

    let vm = init_wasm_vm(wasm_file_path)?;
    match vm.run_func("run", vec![Param::VecU8(&vec![0, 1, 2])])? {
        Ok(_) => Ok(()),
        Err(e) => Err(e.into()),
    }
}
