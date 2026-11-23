use crate::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git, nombre_paquete};

// Golang
pub fn golang(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");
    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(&format!("{}/src", nombre), "main.go", "package main\nimport \"fmt\"\nfunc main() {\n fmt.Println(\"Hola, mundo\") \n}\n");
    // comandos
    ejecutar_comando("go", &["mod","init",&nombre_paquete(nombre)], Some(nombre));
    ejecutar_comando("go", &["mod","tidy"], Some(nombre));
    ejecutar_comando("go", &["fmt","./..."], Some(nombre));
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "bin\n*.exe\n*.test,*.out");
    }
}
