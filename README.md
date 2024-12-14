# rmkit

rmkit is a toolkit set for RMK keyboard firmware.

Now rmkit can be used to generate RMK project directly from `keyboard.toml` and `vial.json`, or interactively.

## Usage

1. Install rmkit:

    ```shell
    cargo install rmkit

    # If you have cargo-binstall, you can use it to speedup the installation:
    cargo binstall rmkit
    ```

2. (option1) Create project from `keyboard.toml` and `vial.json`:

    ```
    rmkit create --keyboard-toml-path keyboard.toml --vial-json-path vial.json
    ```

3. (option2) Create project from project template

    ```
    rmkit init
    ```

    The available project template can be found at [rmk-template](https://github.com/HaoboGu/rmk-template/tree/feat/rework)
