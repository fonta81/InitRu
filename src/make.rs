use super::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git};

// Python
pub fn python(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");
    crear_carpeta(nombre, "tests");
    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(nombre, "requirements.txt", "");
    crear_archivo(&format!("{}/src", nombre), "main.py", "print('hola mundo')\n");
    // comandos
    ejecutar_comando("python3", &["-m", "venv", &format!("{}/.venv", nombre)]);
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "/.venv");
    }
}

// Bash 
pub fn bash(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");
    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(&format!("{}/src", nombre), "main.sh", "#!/bin/bash\necho 'hola mundo'\n");
    // permisos de ejecución
    ejecutar_comando("chmod", &["+x", &format!("{}/src/main.sh", nombre)]);
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "");
    }
}

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
