#!/usr/bin/env bash
# Resolution-only lockfile preview for external dependency intake.
# See .dev/dependencies/README.md
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  dependency-lock-diff.sh <external-crate-name>@<version> [--package <workspace-member>] [--workspace]

Adds the dependency with cargo add, then prints `cargo update --workspace --dry-run`.
Root Cargo.toml and the member manifest are restored on exit.

Modes (cargo add):
  Member-only (default):
    cargo add <crate>@<version> -p <member>
  Workspace root ([workspace.dependencies]):
    cargo add <crate>@<version> --workspace
  Member edge ({ workspace = true }):
    cargo add <crate>@<version> -p <member> --workspace

Default member is openpfe. Use --package with --workspace for the member edge case.

Examples:
  .dev/scripts/dependency-lock-diff.sh serde@1.0 --package openpfe-ipc
  .dev/scripts/dependency-lock-diff.sh tokio@1.48 --workspace
  .dev/scripts/dependency-lock-diff.sh clap@4.5 --workspace --package openpfe
EOF
}

die() {
  echo "error: $*" >&2
  exit 1
}

repo_root() {
  local root
  root="$(git rev-parse --show-toplevel 2>/dev/null)" || die "not inside a git repository"
  printf '%s\n' "$root"
}

member_manifest_path() {
  local package="$1"
  local path

  command -v jq >/dev/null 2>&1 || die "jq is required (brew install jq / apt install jq)"

  path="$(
    cargo metadata --format-version 1 --no-deps |
      jq -r --arg pkg "$package" '.packages[] | select(.name == $pkg) | .manifest_path'
  )"

  [[ -n "$path" && "$path" != "null" ]] || die "workspace member not found: $package"

  printf '%s\n' "$path"
}

# Set in main(); read by EXIT trap (must not be local to main).
manifest_backup=
member_manifest=
member_manifest_backup=

parse_spec() {
  local spec="$1"
  local name version

  [[ "$spec" == *@* ]] || die "expected <external-crate-name>@<version>, got: $spec"

  name="${spec%%@*}"
  version="${spec#*@}"

  [[ -n "$name" && -n "$version" ]] || die "expected <external-crate-name>@<version>, got: $spec"

  printf '%s\n%s\n' "$name" "$version"
}

restore_workspace() {
  if [[ -n "${manifest_backup:-}" && -f "$manifest_backup" ]]; then
    cp "$manifest_backup" Cargo.toml
  fi
  if [[ -n "${member_manifest_backup:-}" && -f "$member_manifest_backup" ]]; then
    cp "$member_manifest_backup" "$member_manifest"
  fi
  rm -f "${manifest_backup:-}" "${member_manifest_backup:-}"
}

main() {
  local spec="${1:-}"
  local package="openpfe"
  local package_explicit=false
  local use_workspace=false

  case "${spec:-}" in
    -h | --help | "")
      usage
      exit "$([[ -n "${spec:-}" ]] && echo 0 || echo 1)"
      ;;
  esac

  shift
  while [[ $# -gt 0 ]]; do
    case "$1" in
      --package)
        [[ $# -ge 2 ]] || die "--package requires a workspace member name"
        package="$2"
        package_explicit=true
        shift 2
        ;;
      --workspace)
        use_workspace=true
        shift
        ;;
      *)
        die "unexpected argument: $1"
        ;;
    esac
  done

  [[ -n "$spec" ]] || die "expected <external-crate-name>@<version>"

  local name version root
  {
    read -r name
    read -r version
  } < <(parse_spec "$spec")

  root="$(repo_root)"
  cd "$root"

  [[ -f Cargo.toml ]] || die "Cargo.toml not found in $root"

  member_manifest="$(member_manifest_path "$package")"
  [[ -f "$member_manifest" ]] || die "member manifest not found: $member_manifest"

  manifest_backup="$(mktemp "${TMPDIR:-/tmp}/cargo-manifest-backup.XXXXXX")"
  member_manifest_backup="$(mktemp "${TMPDIR:-/tmp}/cargo-member-manifest-backup.XXXXXX")"
  cp Cargo.toml "$manifest_backup"
  cp "$member_manifest" "$member_manifest_backup"
  trap restore_workspace EXIT

  if [[ "$use_workspace" == true ]]; then
    if [[ "$package_explicit" == true ]]; then
      cargo add "${name}@${version}" -p "$package" --workspace --quiet
    else
      cargo add "${name}@${version}" --workspace --quiet
    fi
  else
    cargo add "${name}@${version}" -p "$package" --quiet
  fi

  cargo update --workspace --dry-run
}

main "$@"
