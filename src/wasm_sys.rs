#[repr(usize)]
pub enum SysCall {
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
   EMPTY = u32::MAX as usize,
}

impl From<usize> for SysCall {
   fn from(n: usize) -> Self {
      match n {
         0 => Self::KILL,
         1 => Self::CONSOLE_RESET,
         2 => Self::CONSOLE_IN,
         3 => Self::CONSOLE_OUT,
         4 => Self::CONSOLE_GET_TITLE,
         5 => Self::CONSOLE_SET_TITLE,
         10 => Self::GET_PRIORITY,
         11 => Self::SET_PRIORITY,
         12 => Self::GET_HOSTNAME,
         13 => Self::SET_HOSTNAME,
         20 => Self::GET_CONFIG,
         21 => Self::SET_CONFIG,
         22 => Self::MAKE_DIRECTORY,
         23 => Self::DELETE_DIRECTORY,
         24 => Self::RENAME_DIRECTORY,
         25 => Self::SET_DIRECTORY_ACCESS,
         26 => Self::MAKE_FILE,
         27 => Self::DELETE_FILE,
         28 => Self::RENAME_FILE,
         29 => Self::SET_FILE_ACCESS,
         30 => Self::FILE_READ,
         31 => Self::FILE_WRITE,
         50 => Self::ENCRYPT,
         _ => Self::EMPTY,
      }
   }
}
