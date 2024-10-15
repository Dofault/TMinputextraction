use std::borrow::Borrow; 
use std::io;
use std::ptr;

use toy_arms::external::Process;
use winapi::shared::rpcndr::byte;
use winapi::um::memoryapi::ReadProcessMemory;
use winapi::um::xinput::*;
use winapi::shared::minwindef::DWORD;
use super::mutable_xinput_state::*;
use super::function_scheduler::*;
use retour::static_detour;

use winapi;                                         // https://docs.rs/winapi/0.3.9/winapi/index.html -> very useful
use winapi::shared::ntdef::{HANDLE, NULL};          // You need to enable this as feature in Cargo.toml
use winapi::shared::minwindef::{FALSE, LPVOID};
use winapi::um::winnt::PROCESS_ALL_ACCESS;
use winapi::um::processthreadsapi::OpenProcess;     // You need to enable this as feature in Cargo.toml
use winapi::um::handleapi::CloseHandle;             // You need to enable this as feature in Cargo.toml

use sysinfo::{PidExt, ProcessExt, System, SystemExt};

// set up the detour for the XInputGetState function
static_detour! {
  pub static XInputGetStateHook: unsafe extern "system" fn(DWORD, *mut XINPUT_STATE) -> DWORD;
}
pub(crate) static mut MEMORY_INFO: Option<MemoryInfo> = None;

pub static mut MATRIX: Vec<(i16, bool, bool)> = Vec::new();
pub static mut TICK: u64 = 0;

// Structure pour stocker les informations de mémoire
struct MemoryInfo {
    process_handle: HANDLE,
    address: u64,
}

impl MemoryInfo {
    fn new(process_handle: HANDLE, address: u64) -> Self {
        MemoryInfo { process_handle, address }
    }

    fn read_int(&self) -> Option<byte> {
        let mut value: byte = 0;
        let result = unsafe {
            ReadProcessMemory(self.process_handle, self.address as *mut _, &mut value as *mut byte as LPVOID, std::mem::size_of::<byte>(), NULL as *mut usize)
        };
        if result == FALSE {
            println!("ReadProcessMemory failed. Error: {:?}", io::Error::last_os_error());
            None
        } else {
            Some(value)
        }
    }
}

// Variable globale pour stocker les informations de mémoire
// Fonction pour gérer l'état du contrôleur
pub fn handle_controller_state(controller_state: &MutableXInputState, scheduled_functions: &mut Vec<ScheduledFunctionStack>) {
    // Exemple 1: 
    // Lorsque le bouton B est pressé, appuyer sur le bouton X
    /*
    if controller_state.east_button() {
        controller_state.set_west_button(ButtonState::DOWN);

        // Afficher les coordonnées du stick gauche (X, Y)
    }
 */
    let left_stick = controller_state.left_stick_raw();

    // Utiliser la variable globale MEMORY_INFO
    unsafe {

            println!("{} : steer ({}) X({}) RT({})",
                TICK, left_stick.0, controller_state.west_button(), controller_state.right_shoulder()
            );    
            MATRIX.push((left_stick.0, controller_state.west_button(), controller_state.right_shoulder()));
            TICK+=1;
    }
}

fn read_line_address() -> u64 {
    let mut address_buffer = String::new();
    std::io::stdin().read_line(&mut address_buffer).expect("Failed to read line");
    let address_trim = address_buffer.trim().trim_start_matches("0x");
    return u64::from_str_radix(address_trim, 16).unwrap();
}