# Releasing

## 0.0.1 bootstrap

The first crates.io publication must be performed manually because crates.io
Trusted Publishing can only be configured after the crate exists.

After the bootstrap PR is merged:

```console
git switch main
git pull --ff-only
cargo login
cargo publish -p annoda
```

Then configure crates.io **Trusted Publishing** for `annoda`:

- GitHub owner: `Metalymph`
- repository: `annoda`
- workflow file: `release.yml`
- environment: leave empty unless you later add a protected GitHub environment

After Trusted Publishing is configured, create the first tag:

```console
git tag v0.0.1
git push origin v0.0.1
```

The release workflow detects that `annoda 0.0.1` already exists on
crates.io, skips publishing it, and creates the GitHub Release.

## Subsequent releases

1. Change the workspace version in `Cargo.toml`.
2. Merge the release preparation PR.
3. Tag the exact version:

```console
git tag vX.Y.Z
git push origin vX.Y.Z
```

The workflow validates the tag/version match, runs the full quality gate,
packages the crate, obtains a short-lived crates.io token through OIDC Trusted
Publishing, publishes the crate, and creates the GitHub Release.

Never publish automatically from `main`.
