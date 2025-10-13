/*
This product includes software developed under the MIT License.

Copyright (c) 2024-present Frank Zhang
This change is made under the BEAR AI SOFTWARE LICENSE AGREEMENT (Proprietary).
*/

fn main() {
    // WebView2 runtime check for Windows
    #[cfg(target_os = "windows")]
    {
        println!("cargo:warning=Using downloadBootstrapper for WebView2 - runtime will be downloaded if needed");
    }

    tauri_build::build()
}
