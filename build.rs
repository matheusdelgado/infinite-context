use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let lib_dir = PathBuf::from(manifest_dir).join("lib");

    // 1. Caminhos locais do seu .so privado protegido
    println!("cargo:rustc-link-search=native={}", lib_dir.display());
    println!("cargo:rustc-link-search=dependency={}", lib_dir.display());
    println!("cargo:rustc-link-lib=dylib=infinite_context_engine");

    // 2. Caminhos do Python 3.12 no sistema
    println!("cargo:rustc-link-search=native=/usr/lib/python3.12/config-3.12-x86_64-linux-gnu");
    println!("cargo:rustc-link-search=native=/usr/lib/x86_64-linux-gnu");

    // 3. Força a linkagem do Python no binário do benchmark gerado pelo Cargo
    println!("cargo:rustc-link-arg=-Wl,--no-as-needed");
    println!("cargo:rustc-link-arg=-lpython3.12");
    println!("cargo:rustc-link-arg=-Wl,--as-needed");

    // Flags de escape de runtime do Linux
    println!("cargo:rustc-link-arg=-Wl,--allow-shlib-undefined");
    println!("cargo:rustc-link-arg=-Wl,--export-dynamic");
    
    // Configura o RPATH para achar o seu .so local e o python
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../../lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,$ORIGIN/../../lib");
    println!("cargo:rustc-link-arg=-Wl,-rpath,./lib");

    println!("cargo:rerun-if-changed=lib/libinfinite_context_engine.so");
}