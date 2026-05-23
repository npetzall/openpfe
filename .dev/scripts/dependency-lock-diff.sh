#!/usr/bin/env bash
# Resolution-only lockfile preview for external dependency intake.
# See .dev/dependencies/README.md
set -euo pipefail

usage() {
  cat <<'EOF'
Usage:
  dependency-lock-diff.sh <external-crate-name>@<version>

Copies Cargo.toml and Cargo.lock to Cargo-with-<name>.{toml,lock},
runs cargo add and cargo generate-lockfile against those trial paths
(does not modify the workspace lock), then prints a lockfile diff.

Example:
  .dev/scripts/dependency-lock-diff.sh tokio@1.40
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

parse_spec() {
  local spec="$1"
  local name version

  [[ "$spec" == *@* ]] || die "expected <external-crate-name>@<version>, got: $spec"

  name="${spec%%@*}"
  version="${spec#*@}"

  [[ -n "$name" && -n "$version" ]] || die "expected <external-crate-name>@<version>, got: $spec"

  printf '%s\n%s\n' "$name" "$version"
}

main() {
  local spec="${1:-}"

  case "${spec:-}" in
    -h | --help | "")
      usage
      exit "$([[ -n "${spec:-}" ]] && echo 0 || echo 1)"
      ;;
  esac

  [[ $# -eq 1 ]] || die "expected exactly one argument: <external-crate-name>@<version>"

  local name version root manifest lock lock_baseline
  {
    read -r name
    read -r version
  } < <(parse_spec "$spec")

  root="$(repo_root)"
  cd "$root"

  [[ -f Cargo.toml ]] || die "Cargo.toml not found in $root"
  [[ -f Cargo.lock ]] || die "Cargo.lock not found — run 'cargo generate-lockfile' once on the workspace first"

  manifest="Cargo-with-${name}.toml"
  lock="Cargo-with-${name}.lock"

  cp Cargo.toml "$manifest"
  cp Cargo.lock "$lock"

  lock_baseline="$(mktemp "${TMPDIR:-/tmp}/cargo-lock-baseline.XXXXXX")"
  cp "$lock" "$lock_baseline"
  trap 'rm -f "$lock_baseline"' EXIT

  echo "Trial files:"
  echo "  $manifest"
  echo "  $lock"
  echo ""
  echo "Adding ${name}@${version} (resolution only)..."
  echo ""

  cargo add "${name}@${version}" \
    --manifest-path "$manifest" \
    --lockfile-path "$lock"

  cargo generate-lockfile \
    --manifest-path "$manifest" \
    --lockfile-path "$lock"

  echo "# Lockfile delta (resolution only): ${name}@${version}" >&2
  echo "# baseline: $lock_baseline (copy before add)" >&2
  echo "# resolved: $lock" >&2
  echo "" >&2

  if diff -u "$lock_baseline" "$lock"; then
    echo "(no lockfile changes)" >&2
  fi

  echo "" >&2
  echo "Record the diff above in .dev/dependencies/$name/lock-update.md" >&2
  echo "Trial files (gitignored): $manifest $lock" >&2
}

main "$@"
