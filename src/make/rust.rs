use crate::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git};

// Rust
pub fn rust(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");
    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(nombre, "Cargo.toml", &format!("[package]\nname = \"{}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n", nombre));
    crear_archivo(&format!("{}/src", nombre), "main.rs", "fn main() {\n    println!(\"hola mundo\");\n}\n");
    // comandos
    ejecutar_comando("cargo", &["build", "--manifest-path", &format!("{}/Cargo.toml", nombre)]);
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "/target");
    }
}
