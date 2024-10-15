use std::fs::File;
use std::os::windows::io::FromRawHandle;
use std::ptr::null_mut;
use std::{ffi::CString, iter, mem, error::Error, sync::Mutex};
use consoleapi::AllocConsole;
use fileapi::CreateFileA;
use handleapi::INVALID_HANDLE_VALUE;
use winapi::um::*;
use winapi::um::xinput::*;
use winapi::shared::minwindef::{BOOL, DWORD, HINSTANCE, LPVOID, TRUE};
use once_cell::sync::Lazy;

#[macro_use]
mod function_scheduler;

mod handle_controller_state;
mod mutable_xinput_state;

use mutable_xinput_state::MutableXInputState;
use function_scheduler::*;
use handle_controller_state::*;
use winbase::STD_OUTPUT_HANDLE;
use winnt::GENERIC_WRITE;

// type definition for the original XInputGetState function
type XInputGetStateFunc = unsafe extern "system" fn(DWORD, *mut XINPUT_STATE) -> DWORD;

// set up a set of functions to be called on a schedule (see src\helpers\function_scheduler.rs)
static SCHEDULED_FUNCTIONS: Lazy<Mutex<Vec<ScheduledFunctionStack>>> = Lazy::new(|| Mutex::new(vec![]));

unsafe fn main() -> Result<(), Box<dyn Error>> {
  // Find the address of the XInputGetState function
  let address = find_XInputGetState_address().expect("could not find 'XInputGetState' address");
  let target: XInputGetStateFunc = mem::transmute(address);

  // Initialize AND enable the detour
  XInputGetStateHook
    .initialize(target, xinput_get_state_detour)?
    .enable()?;

  Ok(())
}

// XInputGetState function detour
fn xinput_get_state_detour(user_index: DWORD, state_ptr: *mut XINPUT_STATE) -> DWORD {
  // Call the original XInputGetState function so it loads the controller 
  // state into the value referenced by state_ptr
  let to_return =  unsafe { XInputGetStateHook.call(user_index, state_ptr) };

  // Wrap the state_ptr in a MutableXInputState struct so we can modify the controller state safely
  let controller_state = MutableXInputState::from_ptr(state_ptr);

  // Lock the scheduled functions mutex so we can access the scheduled functions
  let mut scheduled_functions = SCHEDULED_FUNCTIONS.lock().unwrap();

  // Call handle_controller_state to modify the controller state
  handle_controller_state(&controller_state, &mut scheduled_functions);

  // If there are any scheduled functions, poll them and clean up the finished ones
  if !scheduled_functions.is_empty() {
    let mut completed: Vec<usize> = vec![];

    // Poll each of the scheduled functions
    for i in 0..scheduled_functions.len() {
      let function_state = scheduled_functions[i].poll(&controller_state);
      match function_state {
        ScheduledFunctionState::Completed => completed.push(i),
        _ => {},
      }
    }

    // Clean up all of the finished functions
    completed.reverse();
    for index in completed {
      scheduled_functions.remove(index);
    }
  }

  to_return
}

// Returns the absolute address of the xinput*.dll module symbol.
#[allow(non_snake_case)]
fn find_XInputGetState_address() -> Option<usize> {
  let xinput14 = "xinput1_4.dll";
  let xinput13 = "xinput1_3.dll";
  let xinput12 = "xinput1_2.dll";
  let xinput11 = "xinput1_1.dll";
  let xinput91 = "xinput9_1_0.dll";

  for lib_name in [xinput14, xinput13, xinput12, xinput11, xinput91].into_iter() {
    if let Some(address) = get_module_symbol_address(lib_name, "XInputGetState") {
      return Some(address);
    }
  }

  None
}

// Returns a module symbol's absolute address.
fn get_module_symbol_address(module: &str, symbol: &str) -> Option<usize> {
  let module = module
    .encode_utf16()
    .chain(iter::once(0))
    .collect::<Vec<u16>>();
  let symbol = CString::new(symbol).unwrap();
  unsafe {
    let handle = libloaderapi::GetModuleHandleW(module.as_ptr());
    match libloaderapi::GetProcAddress(handle, symbol.as_ptr()) as usize {
      0 => None,
      n => Some(n),
    }
  }
}

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "system" fn DllMain(
  _module: HINSTANCE,
  call_reason: DWORD,
  _reserved: LPVOID,
) -> BOOL {
  if call_reason == winnt::DLL_PROCESS_ATTACH {



    // Allouer une nouvelle console
    AllocConsole();

    // Ouvrir un handle pour la console
    let stdout_handle = CreateFileA(
        b"CONOUT$\0".as_ptr() as *const i8, // Console Output
        GENERIC_WRITE,
        0,
        null_mut(),
        winapi::um::fileapi::OPEN_EXISTING,
        0,
        null_mut(),
    );






    main().is_ok() as BOOL
  } else if call_reason == winnt::DLL_PROCESS_DETACH {
    // Désactiver le hook de la fonction XInputGetState
    XInputGetStateHook.disable().unwrap();

    // Libérer la console lorsqu'elle est détachée
    winapi::um::wincon::FreeConsole();

    TRUE
  } else {
    TRUE
  }
}