# Command Reference

## Global Options

| Option | Description |
|--------|-------------|
| `--verbose, -v` | Enable verbose output |
| `--quiet, -q` | Suppress non-essential output |
| `--data-dir <PATH>` | Data directory (default: Datos) |

---

## verify

Runs 21-phase verification on documentation.

```bash
oc_diagdoc verify [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--quick, -Q` | Skip slow phases (V16, V17, V19) |
| `--phase <N>` | Run only specific phase |
| `--json` | JSON output |
| `--progress` | Show progress bar |
| `--cache` | Use sled cache |

---

## stats

Display project statistics dashboard.

```bash
oc_diagdoc stats [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--format <fmt>` | Output format (table/json/yaml) |
| `--by-module` | Group stats by module |
| `--by-status` | Group by status |
| `--by-type` | Group by type |

---

## tree

Display hierarchical document tree.

```bash
oc_diagdoc tree [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--root <ID>` | Root node for visualization |
| `--depth <N>` | Maximum depth |
| `--show-status` | Show document status |
| `--format <FMT>` | Output format (ascii/json/md) |

---

## search

Search in document content and metadata.

```bash
oc_diagdoc search <QUERY> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--fuzzy` | Fuzzy search |
| `--regex` | Use regex pattern |
| `--context <N>` | Lines of context |
| `--yaml-only` | Search only in YAML |

---

## deps

Analyze document dependencies.

```bash
oc_diagdoc deps [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--direction <D>` | Direction: up/down/both |
| `--impact <ID>` | Impact analysis for document |
| `--format <FMT>` | Output format |

---

## links

Analyze internal and external links.

```bash
oc_diagdoc links [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--broken` | Show only broken links |
| `--external` | Include external links |
| `--fix` | Auto-fix broken links |

---

## lint

Static analysis of documentation.

```bash
oc_diagdoc lint [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--fix` | Auto-fix issues |
| `--show-fixes` | Show fix suggestions |
| `--rules <LIST>` | Specific rules to run |

---

## health

Project health dashboard.

```bash
oc_diagdoc health [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--format <FMT>` | Output format |
| `--detailed` | Detailed breakdown |

---

## coverage

Content coverage analysis.

```bash
oc_diagdoc coverage [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--by-module` | Coverage by module |
| `--threshold <N>` | Minimum word count |

---

## trace

Document traceability.

```bash
oc_diagdoc trace [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--to <ID>` | Trace to document |
| `--from <ID>` | Trace from document |

---

## audit

YAML metadata audit.

```bash
oc_diagdoc audit [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--strict` | Strict mode |
| `--json` | JSON output |

---

## report

Generate reports.

```bash
oc_diagdoc report [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--format <FMT>` | Report format |
| `--output <FILE>` | Output file |

---

## diff

Compare project states.

```bash
oc_diagdoc diff [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--from <SNAP>` | Source snapshot |
| `--to <SNAP>` | Target snapshot |
| `--compact` | Compact output |

---

## fix

Correct structural anomalies.

```bash
oc_diagdoc fix [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--dates` | Sync last_updated with filesystem |
| `--hashes` | Recalculate content_hash |
| `--tables` | Fix Nietos column |
| `--dry-run` | Show changes without applying |
| `-v, --verbose` | Show details |

---

## sync

Synchronize metadata.

```bash
oc_diagdoc sync [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--dates` | Sync dates |
| `--hashes` | Sync hashes |
| `--fix-descendants` | Propagate to children |
| `--dry-run` | Simulate changes |

---

## batch

Batch operations on frontmatter.

```bash
oc_diagdoc batch [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--add-field <K=V>` | Add YAML field |
| `--remove-field <K>` | Remove field |
| `--dry-run` | Simulate changes |
| `--progress` | Show progress |

---

## gen

Generate documents.

```bash
oc_diagdoc gen [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--template <T>` | Template to use |
| `--output <PATH>` | Output path |

---

## export

Export documentation.

```bash
oc_diagdoc export [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--format <FMT>` | Format (html/json/latex) |
| `--output <DIR>` | Output directory |
| `--single-file` | Single file output |
| `--zip` | Create ZIP archive |

---

## compress

Compile documentation.

```bash
oc_diagdoc compress [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--preview` | Preview without writing |
| `--pdf` | Generate PDF (requires pandoc) |
| `--output <FILE>` | Output file |

---

## init

Initialize new project.

```bash
oc_diagdoc init <PATH> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--preset <P>` | Preset (minimal/standard/full) |
| `--name <N>` | Project name |

---

## migrate

Migrate project.

```bash
oc_diagdoc migrate [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--from <V>` | Source version |
| `--to <V>` | Target version |

---

## snapshot

Manage snapshots.

```bash
oc_diagdoc snapshot [NAME] [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-l, --list` | List snapshots |
| `-p, --path` | Project path |

---

## restore

Restore from snapshot.

```bash
oc_diagdoc restore <NAME> [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--force` | Force restore |
| `--dry-run` | Simulate restore |

---

## archive

Archive documents.

```bash
oc_diagdoc archive [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--older-than <DAYS>` | Archive older than N days |
| `--dry-run` | Simulate archival |

---

## ci

CI/CD integration.

```bash
oc_diagdoc ci [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--fail-on-warning` | Fail on warnings |
| `--json` | JSON output |

---

## dashboard

Interactive TUI dashboard.

```bash
oc_diagdoc dashboard [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `-f, --filter` | Initial filter: all/errors/warnings/fixable |
| `--quick` | Quick verification |
| `-p, --path` | Data directory path |

### Keybindings

| Key | Action |
|-----|--------|
| `j/↓` | Next issue |
| `k/↑` | Previous issue |
| `a/e/w/f` | Filter modes |
| `q` | Quit |

---

## module

Module operations.

```bash
oc_diagdoc module [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--list` | List modules |
| `--info <ID>` | Module info |

---

## watch

Watch for changes.

```bash
oc_diagdoc watch [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--delay <MS>` | Debounce delay |
| `--command <CMD>` | Command to run on change |

---

## template

Template management.

```bash
oc_diagdoc template [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--list` | List templates |
| `--create <N>` | Create template |

---

## readme

Generate README.

```bash
oc_diagdoc readme [OPTIONS]
```

### Options

| Option | Description |
|--------|-------------|
| `--output <FILE>` | Output file |
| `--full` | Full documentation |

---

## help

Extended help.

```bash
oc_diagdoc help [COMMAND]
```
