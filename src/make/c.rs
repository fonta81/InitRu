use crate::utils::general::{
    InitError, crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git,
};

// C
pub fn c(nombre: &str, git: bool) -> Result<(), InitError> {
    // carpeta principal
    crear_carpeta(nombre, "src");

    // archivos
    crear_archivo(nombre, "README.md", "hello this is my proyec");
    crear_archivo(
        &format!("{}/src", nombre),
        "main.c",
        "#include <stdio.h>\n\nint main(void) {\n    printf(\"Hello, World!\\n\");\n    return 0;\n}\n",
    );
    crear_archivo(
        nombre,
        "Makefile",
        "CC=gcc\nCFLAGS=-Wall -Wextra -std=c11\n\nall:\n\t$(CC) $(CFLAGS) src/main.c -o main\n\nclean:\n\trm -f main\n",
    );

    ejecutar_comando(
        "gcc",
        &["-Wall", "-Wextra", "-std=c11", "src/main.c", "-o", "main"],
        Some(nombre),
    )?;

    if git {
        inicializar_git(nombre)?;
        crear_archivo(nombre, ".gitignore", "main\n*.o\n*.out");
    }

    Ok(())
}
