(module
      (import "host" "add" (func $add (param i32 i32)(result i32)))
      (; (import "host" "empty" (func $empty)) ;)
      (func (export "main") (result i32)
            (; (call $empty) ;)
            (call $add (i32.const 123) (i32.const 456))
      )
)
