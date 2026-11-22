use crate::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git};

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
    ejecutar_comando("python3", &["-m", "venv", &format!("{}/.venv", nombre)], None);
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "/.venv");
    }
}
