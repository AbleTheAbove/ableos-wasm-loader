// use wabt;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ImportsBuilder, ModuleImportResolver, ModuleInstance,
    RuntimeArgs, RuntimeValue, Signature, Trap, ValueType,
};

mod wasm_sys;
use wasm_sys::SysCall;

struct HostFunctions;
impl HostFunctions {
    fn check_signature(&self, index: usize, signature: &Signature) -> bool {
        let (params, ret_ty): (&[ValueType], Option<ValueType>) = match index.into() {
            SysCall::KILL => (&[], None),
            SysCall::EMPTY => (&[], None),
            _ => return false,
        };
        signature.params() == params && signature.return_type() == ret_ty
    }
}
impl Externals for HostFunctions {
    fn invoke_index(
        &mut self,
        index: usize,
        args: RuntimeArgs,
    ) -> Result<Option<RuntimeValue>, Trap> {
        match index.into() {
            /// Take in one arg discard the rest
            SysCall::KILL => {
                println!("hewwo");
                Ok(None)
            }
            /// Do nothing
            SysCall::EMPTY => Ok(None),
            SysCall::EXIT => Ok(None),
            SysCall::CONSOLE_RESET => Ok(None),
            SysCall::CONSOLE_IN => Ok(None),
            SysCall::CONSOLE_OUT => Ok(None),
            SysCall::CONSOLE_GET_TITLE => Ok(None),
            SysCall::CONSOLE_SET_TITLE => Ok(None),
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}
impl ModuleImportResolver for HostFunctions {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        let index = match field_name {
            "kill" => SysCall::KILL as usize,
            "empty" => SysCall::EMPTY as usize,
            "exit" => SysCall::EXIT as usize,
            _ => {
                return Err(Error::Instantiation(format!(
                    "Export {} not found",
                    field_name
                )))
            }
        };

        if !self.check_signature(index, signature) {
            return Err(Error::Instantiation(format!(
                "Export {} has a bad signature",
                field_name
            )));
        }

        Ok(FuncInstance::alloc_host(
            Signature::new(&[][..], None),
            index,
        ))
    }
}

fn main() {
    let wasm_binary = include_bytes!("../wasm/rust.wasm");
    // include_bytes!("../wasm/zig.wasm");
    // include_bytes!("../wasm/c.wasm");
    // include_bytes!("../wasm/wasm.wasm");

    // Load wasm binary and prepare it for instantiation.
    let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

    let imports = ImportsBuilder::new().with_resolver("env", &HostFunctions);

    // Instantiate a module with empty imports and
    // assert that there is no `start` function.
    let instance = ModuleInstance::new(&module, &imports)
        .expect("failed to instantiate wasm module")
        .assert_no_start();

    // Finally, invoke the exported function "test" with no parameters
    // and empty external function executor.
    let result = instance
        .invoke_export("_start", &[], &mut HostFunctions)
        // .with_resolver(&mut HostFunctions)
        .expect("failed to execute export");
    /*.unwrap()
    .try_into()
    .unwrap();*/

    println!(
        "{:?}",
        result // .unwrap()
    );
}
