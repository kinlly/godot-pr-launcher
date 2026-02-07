# PR Manager - Plan de Implementación

## Fase 1: MVP ✅ (Implementado)

### Funcionalidades
- ✅ Doble click en `godot-pr-launcher.exe`
- ✅ Panel que muestra lista de PRs del repo `kinlly/ylbtm`
- ✅ Mostrar título, número, autor y fecha de cada PR
- ✅ Click en PR → ejecuta en background:
  ```bash
  C:\repos\godot.exe --path c:\repos\ylbtm
  ```

### Stack Técnico
- **Frontend**: React 18 + Vite + TailwindCSS
- **Backend**: Rust (Tauri 1.5)
- **GitHub API**: REST API v3
- **Autenticación**: Public API (sin token por ahora)

---

## Fase 2: TODO (Futuro)

### Funcionalidades Planeadas

#### 2.1 Comentarios en PR
- Input de texto para escribir comentarios
- Botón "Send Comment" que usa GitHub API
- Mostrar comentarios existentes

```bash
# Endpoint a usar
POST /repos/kinlly/ylbtm/issues/{pr_number}/comments
```

#### 2.2 Merge PRs
- Botón "Merge PR" visible cuando el PR está listo
- Confirmación antes de hacer merge
- Soporte para merge, squash, y rebase

```bash
# Endpoint a usar
PUT /repos/kinlly/ylbtm/pulls/{pr_number}/merge
```

#### 2.3 GitHub Copilot CLI Integration
- Input para comandos de Copilot
- Ejecutar `gh copilot suggest` con contexto de la PR
- Mostrar respuestas del agente

```bash
# Comandos a integrar
gh copilot suggest "review this PR"
gh copilot explain "what does this code do"
```

#### 2.4 Autenticación
- GitHub OAuth flow
- Almacenar token de forma segura
- Permisos: `repo`, `read:user`

### Arquitectura Fase 2

```
src-tauri/src/
├── main.rs
├── github/
│   ├── mod.rs
│   ├── auth.rs       # OAuth + token storage
│   ├── pulls.rs      # PR operations
│   ├── comments.rs   # Comment operations
│   └── merge.rs      # Merge operations
├── copilot/
│   ├── mod.rs
│   └── cli.rs        # GitHub CLI wrapper
└── godot.rs
```

---

## 🔐 Security Considerations

- Store GitHub tokens in OS keychain (using `keyring` crate)
- Never commit tokens to git
- Use minimal required scopes
- Implement token refresh

---

## 📊 Performance Goals

- Launch time: < 1s
- PR list refresh: < 2s
- Godot launch: < 500ms
- Memory usage: < 50MB

---

## 🧪 Testing Plan

- Unit tests for GitHub API calls
- Integration tests for Godot launcher
- UI tests with Tauri's testing framework
- Manual testing on Ally ROX hardware
