use crate::utils::{crear_archivo, crear_carpeta, ejecutar_comando, inicializar_git, nombre_paquete};

// React
pub fn react(nombre: &str, git: bool) {
    // carpeta principal
    crear_carpeta(nombre, "src");

    // archivos
    crear_archivo(nombre, "README.md", &format!("# {}\n\nProyecto React creado con InitRu.\n\n## Comandos\n\n- `npm run dev`\n- `npm run build`\n- `npm run preview`\n", nombre));
    crear_archivo(nombre, "index.html", &format!("<!doctype html>\n<html lang=\"es\">\n  <head>\n    <meta charset=\"UTF-8\" />\n    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\" />\n    <title>{}</title>\n  </head>\n  <body>\n    <div id=\"root\"></div>\n    <script type=\"module\" src=\"/src/main.jsx\"></script>\n  </body>\n</html>\n", nombre));
    crear_archivo(nombre, "vite.config.js", "import { defineConfig } from 'vite';\nimport react from '@vitejs/plugin-react';\n\nexport default defineConfig({\n  plugins: [react()],\n});\n");
    crear_archivo(nombre, "package.json", &format!("{{\n  \"name\": \"{}\",\n  \"private\": true,\n  \"version\": \"0.0.0\",\n  \"type\": \"module\",\n  \"scripts\": {{\n    \"dev\": \"vite\",\n    \"build\": \"vite build\",\n    \"preview\": \"vite preview\"\n  }}\n}}\n", nombre_paquete(nombre)));
    crear_archivo(&format!("{}/src", nombre), "main.jsx", "import { StrictMode } from 'react';\nimport { createRoot } from 'react-dom/client';\nimport App from './App.jsx';\nimport './index.css';\n\ncreateRoot(document.getElementById('root')).render(\n  <StrictMode>\n    <App />\n  </StrictMode>,\n);\n");
    crear_archivo(&format!("{}/src", nombre), "App.jsx", "import { useState } from 'react';\nimport './App.css';\n\nfunction App() {\n  const [count, setCount] = useState(0);\n\n  return (\n    <main className=\"app\">\n      <h1>hola mundo</h1>\n      <button type=\"button\" onClick={() => setCount((c) => c + 1)}>\n        count is {count}\n      </button>\n    </main>\n  );\n}\n\nexport default App;\n");
    crear_archivo(&format!("{}/src", nombre), "App.css", ".app {\n  display: flex;\n  flex-direction: column;\n  align-items: center;\n  justify-content: center;\n  min-height: 100vh;\n  gap: 1rem;\n  font-family: system-ui, sans-serif;\n}\n\n.app button {\n  padding: 0.5rem 1rem;\n  border-radius: 6px;\n  border: 1px solid #646cff;\n  background: #646cff;\n  color: #fff;\n  cursor: pointer;\n}\n");
    crear_archivo(&format!("{}/src", nombre), "index.css", "*, *::before, *::after {\n  box-sizing: border-box;\n}\n\nbody {\n  margin: 0;\n  background: #242424;\n  color: #fff;\n}\n");

    // dependencias
    ejecutar_comando("npm", &["install", "react", "react-dom"], Some(nombre));
    ejecutar_comando("npm", &["install", "-D", "vite", "@vitejs/plugin-react"], Some(nombre));
    if git {
        inicializar_git(nombre);
        crear_archivo(nombre, ".gitignore", "node_modules\ndist\ndist-ssr\n*.local\n.env\n.DS_Store\nnpm-debug.log*\n");
    }
}
