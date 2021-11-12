#[macro_use]
#[repr(usize)]
declum! {
   SysCall,
   KILL = 0,              // Provide a PID
   CONSOLE_RESET = 1,     // Reset the console
   CONSOLE_IN = 2,        // Console Input
   CONSOLE_OUT = 3,       // Console output
   CONSOLE_GET_TITLE = 4, // Get the console title
   CONSOLE_SET_TITLE = 5, // Set the console title
   GET_PID = 6;           // Get the proccess ID
   PROCESS_INFO = 7;      // Get information about the process
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

   SLEEP=32, // Sleep in milliseconds
   SLEEP_UNTIL=33, // Sleep until this time in milliseconds (if this is below the current time return)
   NANOSLEEP=34, // Sleep in nanoseconds
   NANOSLEEP_UNTIL=35, // Sleep until this time nanoseconds (if this is below the current time return)
   GET_TIME = 36,          // Gets the system time (some derivitive of seconds)
   SET_TIME = 37,          // Sets the system time (some derivitive of seconds)


   // Socket SysCall
   SOCKET_BIND=39, // Used by servers to lock a port
   SOCKET_CONNECT=40,
   SOCKET_DISCONNECT=41,
   SOCKET_SEND=42,
   SOCKET_RECIEVE=43,




   // Security Syscalls
   ENCRYPT = 50;
   EMPTY = u32::MAX as usize
}

macro_rules! declum {
   ($nym:ident, $($variant:ident = $value:expr),*; $catch_all:ident = $max:expr) => {
      pub enum $nym {
         $($variant = $value),*
         $catch_all = $max
      }

      impl From<usize> for $nym {
         fn from(n: usize) -> Self {
            match n {
               $($value => Self::$variant),*
               _ => Self::$catch_all
            }
         }
      }
   };
}
