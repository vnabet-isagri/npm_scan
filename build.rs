// build.rs
fn main() {
    #[cfg(windows)]
    {
        // Pas de macros à définir -> slice vide
        let _ = embed_resource::compile("embed_icon.rc", &[] as &[&str]);
        println!("cargo:rerun-if-changed=embed_icon.rc");
        println!("cargo:rerun-if-changed=app.ico");
    }
}