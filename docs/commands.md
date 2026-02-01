# Command Reference

## Global Options

| Option | Description |
|--------|-------------|
| `--verbose, -v` | Enable verbose output |
| `--quiet, -q` | Suppress non-essential output |
| `--color` | Force colored output |
| `--no-color` | Disable colored output |

---

## verify

Runs 21-phase verification on documentation.

```bash
oc_diagdoc verify <data_dir> [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--quick` | Skip slow phases (16, 17, 19) |
| `--phase <N>` | Run only specific phase |
| `--module <N>` | Limit to specific module |
| `--fail-fast` | Stop on first error |

### Exit Codes
- `0`: All phases passed
- `1`: Verification failures
- `2`: Error during execution

---

## stats

Display project statistics dashboard.

```bash
oc_diagdoc stats [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--format <fmt>` | Output format (table/json/yaml) |
| `--by-module` | Group stats by module |

---

## tree

Display hierarchical document tree.

```bash
oc_diagdoc tree [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--colored` | Enable ANSI colors |
| `--stats` | Show word count per node |
| `--depth <N>` | Maximum depth to display |

---

## search

Search in document content and metadata.

```bash
oc_diagdoc search <query> [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--case-sensitive` | Case-sensitive search |
| `--regex` | Use regex pattern |
| `--yaml-only` | Search only in YAML metadata |

---

## lint

Validate Markdown format and structure.

```bash
oc_diagdoc lint [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--fix` | Auto-fix issues |
| `--backup` | Create backup before fixing |

---

## export

Export documentation to various formats.

```bash
oc_diagdoc export [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--format <fmt>` | Target format (html/json/latex) |
| `--output <dir>` | Output directory |
| `--zip` | Create ZIP archive |

---

## sync

Synchronize metadata and timestamps.

```bash
oc_diagdoc sync [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--dates` | Sync last_updated fields |
| `--hashes` | Update content hashes |
| `--dry-run` | Show changes without applying |

---

## init

Initialize a new documentation project.

```bash
oc_diagdoc init <path> [options]
```

### Options
| Option | Description |
|--------|-------------|
| `--preset <p>` | Template preset (minimal/standard/full) |
| `--name <n>` | Project name |
