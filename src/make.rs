use super::utils::{crear_archivo, crear_carpeta, ejecutar_comando};

pub fn python() {
    crear_carpeta("./proyecto", "src");
    crear_archivo("./proyecto/src", "main.py", "print('hola mundo')\n");
    ejecutar_comando("python3", &["-m", "venv", "./proyecto/.venv"]);
    crear_archivo("./proyecto", "README.md", "hello this is my proyec");
}
