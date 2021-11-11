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
         0x0 => Self::KILL,
         0x1 => Self::CONSOLE_RESET,
         0x2 => Self::CONSOLE_IN,
         0x3 => Self::CONSOLE_OUT,
         0x4 => Self::CONSOLE_GET_TITLE,
         0x5 => Self::CONSOLE_SET_TITLE,
         0xA => Self::GET_PRIORITY,
         0xB => Self::SET_PRIORITY,
         0xC => Self::GET_HOSTNAME,
         0xD => Self::SET_HOSTNAME,
         0x14 => Self::GET_CONFIG,
         0x15 => Self::SET_CONFIG,
         0x16 => Self::MAKE_DIRECTORY,
         0x17 => Self::DELETE_DIRECTORY,
         0x18 => Self::RENAME_DIRECTORY,
         0x19 => Self::SET_DIRECTORY_ACCESS,
         0x1A => Self::MAKE_FILE,
         0x1B => Self::DELETE_FILE,
         0x1C => Self::RENAME_FILE,
         0x1D => Self::SET_FILE_ACCESS,
         0x1E => Self::FILE_READ,
         0x1F => Self::FILE_WRITE,
         0x32 => Self::ENCRYPT,
         _ => Self::EMPTY,
      }
   }
}
