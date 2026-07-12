# agent-vcs

Manual update-review lifecycle fixture.

`agent-vcs` is a protocol-first helper for running a complete deterministic version-control workflow with one agent tool
call. The orchestrating agent keeps decisions and authorization; the Rust binary executes only a requested, predefined
sequence and reports progressive JSONL evidence.

The current build supports two GitHub workflows. `jjstack-github` manages colocated jj/Git repositories through stable
bookmarks, including explicit restacks and stack landing. `git-github` manages ordinary attached Git branches and can
inspect, publish, update, verify, squash-merge, and fast-forward a named destination. Sapling remains unsupported.

## Try the protocol

The package requires Rust 1.85 or later.

```bash
cargo install --path .
mkdir -p "${CODEX_HOME:-$HOME/.codex}/skills"
ln -s "$PWD/agent-vcs" "${CODEX_HOME:-$HOME/.codex}/skills/agent-vcs"
agent-vcs --v1 capabilities
```

Then invoke the installed skill with a workflow-specific request, for example:

```text
Use $agent-vcs with jjstack-github to publish src/lib.rs as review/cli above main in owner/repository.
```

Install or symlink `agent-vcs/` into your Codex skills directory, then ask Codex to use the `agent-vcs` skill with its
workflow knowledge. Direct read-only invocations look like this:

```bash
agent-vcs --v1 --adapter jjstack-github inspect \
  --repository . --github-repository owner/repository --remote origin

agent-vcs --v1 --adapter git-github inspect \
  --repository . --github-repository owner/repository --remote origin
```

Publication takes exact paths, commit prose, PR prose, review ref, and base as explicit arguments. The helper does not
draft, infer, retry mutations, or clean up any of them. A successful call verifies the commit, local and remote review
identity, and PR base/head/SHA before returning. The Git adapter uses path-only commits so unrelated index and worktree
state survives publication. A failure stops at the first discrepancy and preserves partial-effect evidence for the
orchestrator to assess. The sole observation retry is a bounded jj update check when GitHub still reports the exact
pre-push PR head; every read attempt remains protocol evidence.

Lifecycle operations remain explicit. Stack commands consume a complete bottom-to-top JSON manifest rather than
discovering descendants, merges require the literal `squash` strategy and exact expected head SHAs, and replacement
checks may wait only within a caller-supplied timeout. The helper never deletes review bookmarks or rolls back a partial
landing.

Stdout from an accepted operation is newline-delimited v1 JSON. Help and version text are exceptions. CLI errors and
human-readable tracing use stderr. Missing `--v1` and unknown protocol flags fail with exit code 2 before any JSON event
or child command.

## Why the boundary exists

One high-level invocation replaces repeated agent/tool round trips, but the orchestrator keeps every decision: scope,
prose, refs, stack shape, merge authorization, and recovery. The helper runs only a predefined shell-free sequence and
checks each mutation against independent machine-readable state. It has no companion agent because repository access,
credentials, and user authority should remain in the calling session.

Failures are transparent rather than transactional. The helper neither retries nor rolls back; its final event records
completed, failed, and skipped steps plus confirmed, absent, mismatched, or unknown effects. Recovery remains a manual
workflow decision because an automatic reset, cleanup, ref deletion, or force-push could destroy unrelated work.

## Support and trust boundary

| Adapter          | Operations                                                                        | Native assumptions                            |
| ---------------- | --------------------------------------------------------------------------------- | --------------------------------------------- |
| `jjstack-github` | inspect, publish, publish-next, verify, update, restack, merge, merge-stack, sync | Colocated jj/Git with stable review bookmarks |
| `git-github`     | inspect, publish, verify, update, merge, sync                                     | Attached Git branches; path-only commits      |

Sapling, detached Git HEAD, arbitrary external adapters, automatic scope discovery, rollback, and cleanup are not
supported. The process inherits the caller's filesystem and GitHub credentials; use it only for repositories and forge
identities already within that caller's authority. Git pushes are bound to a verified GitHub URL, and GitHub CLI calls
pin the explicit repository and host.

The default executable is `agent-vcs`. Environment-specific instructions may name another executable, but the skill must
first confirm that it exposes the same `--v1` commands and JSONL protocol. Runtime capability output remains the
authority for a particular build.

The complete caller-visible design is in [SPEC.md](SPEC.md). Skill-side protocol, recovery, and adapter guidance lives
under [agent-vcs/references](agent-vcs/references).
