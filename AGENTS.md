# Repository instructions

Follow the [Shuyra maintenance standard](https://github.com/shuyra-arche/vanllia/blob/main/CONTRIBUTING.md).
Use README.md for local build and test entry points.

- Preserve wire identities, ownership, rollback and reclamation constraints.
- Keep target builds and host test workspaces distinct; neither host tests nor
  ELF builds establish runtime success.
- Use apply_patch for edits; retain assertions and failure criteria.
- Keep dependency pins and locks unchanged in documentation-only work.
- Review the final diff and report actual commands and results in the PR.
