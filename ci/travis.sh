#!/bin/bash

set -o errexit -o nounset

cargo test --verbose

BRANCH=$(if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then echo $TRAVIS_BRANCH; else echo $TRAVIS_PULL_REQUEST_BRANCH; fi)

if [ "$BRANCH" == "master" ]; then
    echo "uploading crate docs"

    cargo doc -p val

    REV=$(git rev-parse --short HEAD)
    cd target/doc
    git init
    git remote add upstream "https://$GH_TOKEN@github.com/KodrAus/val"
    git config user.name "val"
    git config user.email "travis@val.rs"
    git add -A .
    git commit -qm "Build docs at ${TRAVIS_REPO_SLUG}@${REV}"

    echo "Pushing gh-pages to GitHub"
    git push -q upstream HEAD:refs/heads/gh-pages --force
fi
