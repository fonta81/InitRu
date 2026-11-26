# InitRu

Una herramienta CLI rapida y sencilla para inicializar proyectos, escrita en Rust.

[![Licencia: MIT](https://img.shields.io/badge/Licencia-MIT-yellow.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-Edición_2024-orange.svg)](https://www.rust-lang.org/)

**Idiomas:** [English](README.md) | **Español**

---

## Descripcion General

InitRu es una herramienta de linea de comandos ligera desarrollada en Rust para automatizar la creacion de la estructura base (scaffolding) de proyectos de software. Permite crear proyectos estructurados con sus herramientas base y configuracion opcional de Git con un solo comando.

---

## Caracteristicas

- **Inicializacion Rapida:** Genera la base de nuevos proyectos al instante.
- **Soporte para Python:**
  - Estructura estandar de directorios y archivos de inicio.
  - Creacion automatica de entorno virtual (`python3 -m venv .venv`).
- **Soporte para Django:**
  - Crea `<nombre>/manage.py` y el paquete Django en `<nombre>/<nombre>/`.
  - Prepara un entorno aislado `.venv`, instala Django y ejecuta `startproject`.
  - Requiere Python 3 y acceso a paquetes de Python; el nombre debe ser un identificador valido de Python.
  - Con `-g`, inicializa Git e ignora el entorno virtual, la cache de Python y la base SQLite predeterminada.
- **Soporte para Bash:**
  - Crea el directorio `src/` y un script ejecutable `src/main.sh`.
- **Soporte para Rust:**
  - Crea el directorio `src/`, `Cargo.toml` y `src/main.rs`.
  - Compila el proyecto con `cargo build`.
- **Soporte para JavaScript:**
  - Crea el directorio `src/`, `package.json` (módulos ES y script `start`) y `src/main.js`.
- **Soporte para TypeScript:**
  - Crea un proyecto TypeScript para Node.js con `src/main.ts`, `tsconfig.json` estricto y scripts npm para compilar e iniciar.
  - Instala TypeScript y compila el proyecto en `dist/`.
- **Soporte para React:**
  - Crea un proyecto React con Vite (`index.html`, `vite.config.js`, `src/main.jsx`, `src/App.jsx`).
  - Instala automáticamente `react`, `react-dom`, `vite` y `@vitejs/plugin-react` (`npm install`).
- **Integracion con Git opcional (`-g`):**
  - Inicializa el repositorio con `git init`.
  - Genera automaticamente `.gitignore` adaptado al lenguaje.


---

## Modo de Uso

```text
Uso: InitRu [OPCIONES] [COMANDO]

Comandos:
  python  Crea un nuevo proyecto en Python
  django  Crea un nuevo proyecto en Django
  bash    Crea un nuevo proyecto en Bash
  rust    Crea un nuevo proyecto en Rust
  javascript  Crea un nuevo proyecto en JavaScript
  typescript  Crea un nuevo proyecto en TypeScript
  react   Crea un nuevo proyecto en React (Vite)
  help    Muestra este mensaje o la ayuda del subcomando

Opciones:
  -g          Inicializa un repositorio git
  -h, --help  Muestra la ayuda
```

### Ejemplos

1. **Crear un proyecto Python estandar:**
   ```bash
   InitRu python mi_proyecto
   ```

2. **Crear un proyecto Python con Git inicializado:**
   ```bash
   InitRu -g python mi_proyecto
   ```

3. **Crear un proyecto Bash:**
   ```bash
   InitRu bash mi_script
   ```

4. **Crear un proyecto Bash con Git inicializado:**
   ```bash
   InitRu -g bash mi_script
   ```

5. **Crear un proyecto Rust:**
   ```bash
   InitRu rust mi_crate
   ```

6. **Crear un proyecto Rust con Git inicializado:**
   ```bash
   InitRu -g rust mi_crate
   ```

7. **Crear un proyecto JavaScript:**
   ```bash
   InitRu javascript mi_app
   ```

8. **Crear un proyecto JavaScript con Git inicializado:**
   ```bash
   InitRu -g javascript mi_app
   ```

9. **Crear un proyecto React:**
   ```bash
   InitRu react mi_app
   ```

10. **Crear un proyecto TypeScript:**
    ```bash
    InitRu typescript mi_app
    ```

11. **Crear un proyecto React con Git inicializado:**
    ```bash
    InitRu -g react mi_app
    ```

12. **Crear un proyecto Django:**
    ```bash
    InitRu django mi_sitio
    ```

13. **Crear un proyecto Django con Git inicializado:**
    ```bash
    InitRu -g django mi_sitio
    ```

14. **Consultar la ayuda:**
    ```bash
    InitRu -h
    InitRu python -h
    InitRu django -h
    InitRu bash -h
    InitRu rust -h
    InitRu javascript -h
    InitRu typescript -h
    InitRu react -h
    ```

---

## Licencia

Este proyecto esta bajo la **Licencia MIT**. Consulta el archivo [LICENSE](LICENSE) para mas detalles.

Copyright (c) 2026 fonta81.
