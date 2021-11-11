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
         SysCall::KILL => (&[ValueType::I32, ValueType::I32], Some(ValueType::I32)),
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
         SysCall::KILL => {
            let a: u32 = args.nth_checked(0)?;
            let b: u32 = args.nth_checked(1)?;
            let result = a + b;

            Ok(Some(RuntimeValue::I32(result as i32)))
         }
         SysCall::CONSOLE_RESET => {}
         SysCall::CONSOLE_IN => {}
         SysCall::CONSOLE_OUT => {}
         SysCall::CONSOLE_GET_TITLE => {}
         SysCall::CONSOLE_SET_TITLE => {}

         _ => panic!("Unimplemented function at {}", index),
      }
   }
}
impl ModuleImportResolver for HostFunctions {
   fn resolve_func(&self, field_name: &str, signature: &Signature) -> Result<FuncRef, Error> {
      let index = match field_name {
         "add" => SysCall::KILL as usize,
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
         Signature::new(&[ValueType::I32, ValueType::I32][..], Some(ValueType::I32)),
         index,
      ))
   }
}

fn main() {
   // Parse WAT (WebAssembly Text format) into wasm bytecode.
   let wasm_binary = wabt::wat2wasm(include_str!("../wasm/test.wat"));

   let wasm_binary = match wasm_binary {
      Ok(abc) => abc,
      Err(abc) => {
         println!("{}", abc);
         return;
      }
   };

   // .expect("failed to parse wat");

   // Load wasm binary and prepare it for instantiation.
   let module = wasmi::Module::from_buffer(&wasm_binary).expect("failed to load wasm");

   let imports = ImportsBuilder::new().with_resolver("host", &HostFunctions);

   // Instantiate a module with empty imports and
   // assert that there is no `start` function.
   let instance = ModuleInstance::new(&module, &imports)
      .expect("failed to instantiate wasm module")
      .assert_no_start();

   // Finally, invoke the exported function "test" with no parameters
   // and empty external function executor.
   let result: i32 = instance
      .invoke_export("main", &[], &mut HostFunctions)
      // .with_resolver(&mut HostFunctions)
      .expect("failed to execute export")
      .unwrap()
      .try_into()
      .unwrap();

   println!(
      "{:?}",
      result // .unwrap()
   );
}
