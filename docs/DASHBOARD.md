# 🖥️ Dashboard TUI

Interfaz TUI interactiva para visualizar issues de verificación.

## Descripción

El comando `dashboard` proporciona una interfaz de usuario basada en terminal (TUI) usando `ratatui` y `crossterm` para visualizar de forma interactiva los resultados de verificación.

## Uso

```bash
oc_diagdoc dashboard [OPTIONS]
```

## Opciones

| Flag | Descripción |
|------|-------------|
| `-p, --path <PATH>` | Ruta al directorio de datos |
| `-f, --filter <FILTER>` | Filtro inicial: `all`, `errors`, `warnings`, `fixable` (default: all) |
| `--quick` | Ejecutar verificación rápida |
| `-v, --verbose` | Modo verbose |
| `-q, --quiet` | Modo silencioso |

## Keybindings

| Tecla | Acción |
|-------|--------|
| `j` / `↓` | Siguiente issue |
| `k` / `↑` | Anterior issue |
| `a` | Filtro: All |
| `e` | Filtro: Errors |
| `w` | Filtro: Warnings |
| `f` | Filtro: Fixable |
| `q` | Salir |

## Ejemplos

```bash
# Dashboard básico
oc_diagdoc dashboard

# Filtrar solo errores al inicio
oc_diagdoc dashboard --filter errors

# Dashboard con verificación rápida
oc_diagdoc dashboard --quick

# Dashboard en directorio específico
oc_diagdoc dashboard -p ./mi-proyecto/Datos
```

## Interfaz

```
┌─ oc_diagdoc Dashboard ─────────────────────────────┐
│                                                     │
│  📊 Verificación: 1387 archivos                    │
│  ✅ Pasados: 95%  │  ❌ Errores: 15  ⚠️ Warnings: 42│
│                                                     │
│  ┌─ Issues ───────────────────────────────────────┐│
│  │ ❌ [V008] 1.2.3 doc.md: fecha desincronizada   ││
│  │ ⚠️ [L002] 3.1.md: header duplicado             ││
│  │ ⚠️ [L009] config.md: línea muy larga           ││
│  └────────────────────────────────────────────────┘│
│                                                     │
│  [a]ll [e]rrors [w]arnings [f]ixable [q]uit        │
└─────────────────────────────────────────────────────┘
```

## Dependencias

- `ratatui 0.29`
- `crossterm 0.28`

## Desde v3.1.0
