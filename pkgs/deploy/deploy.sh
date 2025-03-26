#! @runtimeShell@
# shellcheck shell=bash

set -euo pipefail
shopt -s inherit_errexit

tmpdir=$(mktemp -d)

cleanup() {
    rm -rf "$tmpdir"
}
trap cleanup EXIT

export PATH=@path@:$PATH

nix_flags=()
while getopts ":v" opt; do
    case $opt in
        v)
            set -x
            nix_flags+=(--verbose -L)
            ;;
        \?)
            echo "Invalid option: -$OPTARG" >&2
            exit 1
            ;;
    esac
done

if [ $# -lt 1 ]; then
  echo "Error: Please provide the target machine" >&2
  exit 1
fi

target=$1
operation=${2:-switch}

sha=$(git rev-parse HEAD)
git diff --name-only HEAD > "$tmpdir/files"

git worktree add --quiet --force "$tmpdir/deploy" "$sha"
rsync --quiet --files-from="$tmpdir/files" -l . "$tmpdir/deploy"
pushd "$tmpdir/deploy" >/dev/null
git add --pathspec-from-file="$tmpdir/files"
popd >/dev/null

# Build and copy the system configuration
echo "== Calculating system" >&2
drv=$(nix --extra-experimental-features "nix-command flakes" eval --raw "git+file://$tmpdir/deploy?allRefs=1#nixosConfigurations.scan-server.config.system.build.toplevel.drvPath" --show-trace --fallback "${nix_flags[@]}")

echo "== Copying build to $target" >&2
nix --builders-use-substitutes --extra-experimental-features "nix-command flakes" copy --derivation --to "ssh-ng://$target" "$drv^*"

echo "$drv^*"

echo "Building scan-server on target" >&2
ssh "$target" -- "sudo nix --extra-experimental-features \"nix-command flakes\" build --profile /nix/var/nix/profiles/system \"$drv^*\""

echo "== Switching to scan-server" >&2
ssh "$target" -- sudo /nix/var/nix/profiles/system/bin/switch-to-configuration "$operation"