# Architecture Rules Template

Replace this template with project-specific architecture constraints.

## Layers

Document the intended boundaries.

```txt
UI / API / Application / Domain / Infrastructure
```

## Allowed Dependencies

- `LAYER_A` may depend on:
- `LAYER_B` may not depend on:

## File Routing

| Concern | Location |
|---|---|
| UI components | `PATH_REPLACE_ME` |
| API routes | `PATH_REPLACE_ME` |
| Domain logic | `PATH_REPLACE_ME` |
| Persistence | `PATH_REPLACE_ME` |
| Tests | `PATH_REPLACE_ME` |

## Refactor Rules

- Prefer local changes over broad rewrites.
- Preserve public contracts unless the task explicitly changes them.
- Update docs when architecture changes.
- Add a decision record for important tradeoffs.

