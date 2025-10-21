 warning: unused import: `std::os::windows::process::CommandExt`
  --> src\process_helper.rs:32:13
   |
32 |         use std::os::windows::process::CommandExt as _;
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(unused_imports)]` on by default

warning: `bear-llm-ai` (lib) generated 1 warning
    Finished `release` profile [optimized] target(s) in 13m 56s
    Built application at: D:\a\project-gouda\project-gouda\src-tauri\target\release\BEAR LLM AI.exe
    Info Verifying NSIS package
    Downloading https://github.com/tauri-apps/binary-releases/releases/download/nsis-3/nsis-3.zip
    Info validating hash
    Info extracting NSIS
    Downloading https://github.com/tauri-apps/nsis-tauri-utils/releases/download/nsis_tauri_utils-v0.4.1/nsis_tauri_utils.dll
    Info validating hash
    Info Target: x64
    Running makensis.exe to produce D:\a\project-gouda\project-gouda\src-tauri\target\release\bundle\nsis\BEAR LLM AI_0.0.10_x64-setup.exe
warning: !warning: MUI_LANGUAGE[EX] should be inserted after the MUI_[UN]PAGE_* macros (macro:MUI_LANGUAGEEX:6)
Error: can't change compressor after data already got compressed or header already changed!
Error in script "D:\a\project-gouda\project-gouda\src-tauri\target\release\nsis\x64\installer.nsi" on line 80 -- aborting creation process
failed to bundle project: `The system cannot find the file specified. (os error 2)`
    Error failed to bundle project: `The system cannot find the file specified. (os error 2)`
 ELIFECYCLE  Command failed with exit code 1.
Error: Command "pnpm ["tauri","build"]" failed with exit code 1