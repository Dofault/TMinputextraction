use dll_syringe::{Syringe, process::OwnedProcess};
use toy_arms::{detect_keypress, VirtualKeyCode};

const EXE_NAME: &str = "Trackmania";
const DLL_NAME: &str = "rocket_league_hook.dll";

fn main() {
    // find the target process by name
    let target_process = OwnedProcess::find_first_by_name(EXE_NAME).unwrap();
    let syringe = Syringe::for_process(target_process);

    // inject the payload into the target process
    let file_path = format!("{}/{}/{}", env!("CARGO_MANIFEST_DIR"), "target/debug/deps/", DLL_NAME);
    let mut injected_payload = syringe.inject(&file_path).unwrap();

    // if cfg!(debug_assertions) {
    //     println!("Debugging enabled");
    // } else {
    //     println!("Debugging disabled");
    // }

    println!("");
    println!("DLL injected successfully!");
    println!("  [F9]  to reload the DLL");
    println!("  [F12] to unload the DLL");
    println!("");

    loop {
        // Reload the DLL if F9 is pressed
        if detect_keypress(VirtualKeyCode::VK_F9) {
            print!("Reloading DLL...");
            syringe.eject(injected_payload).unwrap();
            println!(" Done!");
            injected_payload = syringe.inject(&file_path).unwrap();
        }

        // Unload the DLL if F12 is pressed
        if detect_keypress(VirtualKeyCode::VK_F12) {
            print!("Unloading DLL...");
            syringe.eject(injected_payload).unwrap();
            println!(" Done!");
            break;
        }
    }
}