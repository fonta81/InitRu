use super::utils::{crear_archivo, crear_carpeta, ejecutar_comando};

pub fn python() {
    // carpeta principal
    crear_carpeta("./proyecto", "src");
    // archivos
    crear_archivo("./proyecto", "README.md", "hello this is my proyec");
    crear_archivo("./proyecto", ".gitignore", "/.venv");
    crear_archivo("./proyecto/src", "main.py", "print('hola mundo')\n");
    // comandos
    ejecutar_comando("python3", &["-m", "venv", "./proyecto/.venv"]);
    ejecutar_comando("git", &["init","./proyecto"]);
}
