use crate::utils::general::{crear_archivo, crear_carpeta, inicializar_git};

// JavaScript
pub fn javascript(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");

    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(nombre, "package.json", &format!("{{\n  \"name\": \"{}\",\n  \"version\": \"1.0.0\",\n  \"type\": \"module\",\n  \"main\": \"src/main.js\",\n  \"scripts\": {{\n    \"start\": \"node src/main.js\"\n  }}\n}}\n", nombre));
    crear_archivo(&format!("{}/src", nombre), "main.js", "console.log('hola mundo');\n");

    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "/node_modules");
    }
}
