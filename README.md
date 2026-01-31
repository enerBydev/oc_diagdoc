# 🦀⚛️☢️ oc_diagdoc v3.0-NUCLEAR

> **Motor algorítmico nuclear para documentación OnlyCarNLD**

## Instalación

```bash
cargo build --release --features full
```

## Uso

```bash
# Verificar documentación
oc_diagdoc verify Datos/

# Estadísticas
oc_diagdoc stats

# Cobertura
oc_diagdoc coverage --min-words 300

# Lint
oc_diagdoc lint --fix
```

## Comandos Disponibles (29)

### Analíticos
- `verify` - Validación integral
- `stats` - Estadísticas
- `search` - Búsqueda
- `deps` - Dependencias
- `tree` - Árbol jerárquico

### Diagnóstico
- `lint` - Validación Markdown
- `health` - Score de salud
- `coverage` - Cobertura de contenido
- `trace` - Trazabilidad
- `audit` - Auditoría forense

### Generación
- `gen` - Generación automática
- `template` - Templates
- `export` - Exportación
- `compress` - Compilación

### Producción
- `init` - Inicialización
- `migrate` - Migración
- `snapshot` - Snapshots
- `restore` - Restauración
- `ci` - CI/CD

## Licencia

MIT - enerBydev
