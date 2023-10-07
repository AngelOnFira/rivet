# Buggy Cache Purging

Running `bolt up` or `bolt check` may frequently rebuild services if there is a problem with the Bolt generation code. These problems are usually not obvious.

## Cause

Two factors can cause this:

- Bolt frequently regenerates files (e.g. _lib/bolt/core/src/tasks/gen.rs_)
- Rivet services often use _build.rs_ scripts that can trigger rebuilds


## Solution

Run this command:

```
CARGO_LOG=cargo::core::compiler::fingerprint=info bolt check
```

Search the logs for lines starting with `stale:`. This is what is causing a rebuild.

If this is a file generated by Bolt, ensure that Bolt is using `write_if_different` when writing the file.

If the file is JSON and uses a `HashMap`, then try encoding the file using `cjson` for a deterministic output.
