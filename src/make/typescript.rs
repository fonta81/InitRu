use crate::utils::general::{
    crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git, nombre_paquete,
};

pub fn typescript(nombre: &str, git: bool) {
    crear_carpeta(nombre, "src");

    crear_archivo(
        nombre,
        "README.md",
        &format!(
            "# {nombre}\n\nProyecto TypeScript creado con InitRu.\n\n## Comandos\n\n- `npm run build`\n- `npm start`\n"
        ),
    );
    crear_archivo(
        nombre,
        "package.json",
        &format!(
            "{{\n  \"name\": \"{}\",\n  \"version\": \"1.0.0\",\n  \"type\": \"module\",\n  \"scripts\": {{\n    \"build\": \"tsc\",\n    \"start\": \"node dist/main.js\"\n  }},\n  \"devDependencies\": {{\n    \"typescript\": \"^5.0.0\"\n  }}\n}}\n",
            nombre_paquete(nombre)
        ),
    );
    crear_archivo(
        nombre,
        "tsconfig.json",
        "{\n  \"compilerOptions\": {\n    \"target\": \"ES2022\",\n    \"module\": \"NodeNext\",\n    \"moduleResolution\": \"NodeNext\",\n    \"rootDir\": \"./src\",\n    \"outDir\": \"./dist\",\n    \"strict\": true,\n    \"esModuleInterop\": true,\n    \"forceConsistentCasingInFileNames\": true,\n    \"skipLibCheck\": true\n  },\n  \"include\": [\"src/**/*.ts\"]\n}\n",
    );
    crear_archivo(
        &format!("{nombre}/src"),
        "main.ts",
        "console.log('hola mundo');\n",
    );

    ejecutar_comando("npm", &["install"], Some(nombre));

    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "node_modules/\ndist/\n");
    }
}
