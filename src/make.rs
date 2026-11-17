use crate::utils::inicializar_git;

use super::utils::{crear_archivo, crear_carpeta, ejecutar_comando};

pub fn python(git:bool) {
    // carpeta principal
    crear_carpeta("./proyecto", "src");
    crear_carpeta("./proyecto", "tests");
    // archivos
    crear_archivo("./proyecto", "README.md", "hello this is my proyec");
    crear_archivo("./proyecto", "requirements.txt", "");
    crear_archivo("./proyecto/src", "main.py", "print('hola mundo')\n");
    // comandos
    ejecutar_comando("python3", &["-m", "venv", "./proyecto/.venv"]);
    if git {
        inicializar_git("./proyecto");
        crear_archivo("./proyecto", ".gitignore", "/.venv");
    }
}
