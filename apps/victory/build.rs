fn main() {
    // the Agility SDK exports the D3D12 loader looks for in the exe
    println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKVersion");
    println!("cargo:rustc-link-arg-bins=/EXPORT:D3D12SDKPath");
}
