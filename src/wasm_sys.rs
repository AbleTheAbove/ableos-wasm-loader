macro_rules! syscall_enum {
   (@get_last $Variant:ident) => {
      Self::$Variant
   };
   (@get_last $Variant:ident, $($VariantTail:ident),*) => {
      syscall_enum![@get_last $($VariantTail),*]
   };
   ($($Variant:ident = $Value:expr,)*) => {
      #[repr(usize)]
      pub enum SysCall {
         $($Variant = $Value),*
      }

      impl From<usize> for SysCall {
         fn from(n: usize) -> Self {
            match n {
               $($Value => Self::$Variant),*,
               _ => syscall_enum![@get_last $($Variant),*]
            }
         }
      }
   };
   ($($Variant:ident = $Value:expr),* ) => {
      syscall_enum!($($Variant = $Value,)* );
   };
}

syscall_enum! {
   KILL = 0,              // Provide a PID
   CONSOLE_RESET = 1,     // Reset the console
   CONSOLE_IN = 2,        // Console Input
   CONSOLE_OUT = 3,       // Console output
   CONSOLE_GET_TITLE = 4, // Get the console title
   CONSOLE_SET_TITLE = 5, // Set the console title

   //scheduler Related Syscals
   GET_PRIORITY = 10, // Get scheduler priority
   SET_PRIORITY = 11, // Set scheduler priority
   //
   GET_HOSTNAME = 12,
   SET_HOSTNAME = 13,

   //File Related syscalls
   GET_CONFIG = 20, // Get config
   SET_CONFIG = 21, // Set the config
   //
   MAKE_DIRECTORY = 22,       //
   DELETE_DIRECTORY = 23,     //
   RENAME_DIRECTORY = 24,     //
   SET_DIRECTORY_ACCESS = 25, //
   //
   MAKE_FILE = 26,       //
   DELETE_FILE = 27,     //
   RENAME_FILE = 28,     //
   SET_FILE_ACCESS = 29, //

   FILE_READ = 30,
   FILE_WRITE = 31,

   // Security Syscalls
   ENCRYPT = 50,
   EMPTY = 0xFFFF,
}
