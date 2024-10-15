<div id="top"></div>

<!-- OVERVIEW -->
# Trackmania extraction input

This project is based on [rusty-xinput-detour](https://github.com/DavidAngell/rusty-xinput-detour) it allows displaying a controller's inputs in Trackmania (Xbox One, should work with other controllers but not with a keyboard).

<!-- RUNNING EXAMPLE -->
## Running Example
### Prerequisites

You will need Rust and Trackmania to view the example. [Trackmania is free on Epic Games](https://store.epicgames.com/en-US/p/rocket-league) (not sponsored).
* [Rust](https://www.npmjs.com/)
  ```sh
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  ```

### Installation

1. Build the DLL
   ```sh
   cd xinput_detour_dll/
   cargo build
   ```
2. Run the injector script (must be in the root directory)
   ```sh
   cd ..
   cargo run
   ```

## Usage
### Editing [main.rs](src/main.rs)
- Change the value of ```EXE_NAME``` to match the name of the exe you want to detour
- IMPORTANT: if you change the lib name in Cargo.toml it will change the name of the genrated dll. You will need to change ```DLL_NAME``` at the top of main.rs to match whatever lib name you choose

### Handling Controller Actions
See examples in [handle_controller_state.rs](xinput_detour_dll/src/handle_controller_state.rs) to get an understanding of how function scheduler works.

<p align="right">(<a href="#readme-top">back to top</a>)</p>


<!-- CONTRIBUTING -->
## Contributing

If you have a suggestion that would make this program better, please fork the repo and create a pull request. Thank You.

1. Fork the Project
2. Create your Feature Branch (`git checkout -b feature/AmazingFeature`)
3. Commit your Changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the Branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

<p align="right">(<a href="#top">back to top</a>)</p>

<!-- ACKNOWLEDGMENTS -->
## Acknowledgments

* [Othneil Drew's README Template](https://github.com/othneildrew/Best-README-Template)
