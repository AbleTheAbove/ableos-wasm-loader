use wabt;
use wasmi::{
    Error, Externals, FuncInstance, FuncRef, ImportsBuilder, ModuleImportResolver, ModuleInstance,
    RuntimeArgs, RuntimeValue, Signature, Trap, ValueType,
};

mod wasm_sys;
use wasm_sys::SysCall;

pub const KILL: usize = 0;

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
            // SysCall::CONSOLE_RESET => {}
            // SysCall::CONSOLE_IN => {}
            // SysCall::CONSOLE_OUT => {}
            // SysCall::CONSOLE_GET_TITLE => {}
            // SysCall::CONSOLE_SET_TITLE => {}
            _ => panic!("Unimplemented function at {}", index),
        }
    }
}
impl ModuleImportResolver for HostFunctions {
    fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
        let index = match field_name {
            "kill" => SysCall::KILL as usize,
            "empty" => SysCall::EMPTY as usize,
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
    // Parse WAT (WebAssembly Text format) into wasm bytecode.
    // let wasm_binary = wabt::wat2wasm(include_str!("../wasm/test.wat"));
    let wasm_binary = //wabt::wat2wasm(
        include_bytes!("../wasm/ableos-wasm-test.wasm"); //.unwrap();
                                                         // );
                                                         /*
                                                             let wasm_binary = match wasm_binary {
                                                                 Ok(abc) => abc,
                                                                 Err(abc) => {
                                                                     println!("{}", abc);
                                                                     return;
                                                                 }
                                                             };
                                                         */
    // .expect("failed to parse wat");

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
