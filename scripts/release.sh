#!/bin/bash
set -euo pipefail

TAG=${1:-}

if [[ -z "$TAG" ]]; then
    echo "Usage: $0 <tag>"
    echo "Example: $0 v0.2.1"
    exit 1
fi

# Validate tag format (vx.y.z)
if [[ ! "$TAG" =~ ^v[0-9]+\.[0-9]+\.[0-9]+$ ]]; then
    echo "Error: Tag must be in format vx.y.z (e.g., v0.2.1)"
    exit 1
fi

# Extract version from tag (remove 'v' prefix)
VERSION="${TAG#v}"

echo "Releasing ${TAG}..."

# Update version in Cargo.toml ([package] section only)
sed '/^\[package\]/,/^\[/{ s/^version = ".*"/version = "'"${VERSION}"'"/; }' Cargo.toml >Cargo.toml.tmp && mv Cargo.toml.tmp Cargo.toml

# Update Cargo.lock
cargo generate-lockfile

# Commit changes
git add Cargo.toml Cargo.lock
git commit -m "chore: release ${TAG}"

# Create and push tag
git tag "${TAG}"
git push origin main
git push origin "${TAG}"

echo "Released ${TAG} successfully!"
