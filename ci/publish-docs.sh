#!/bin/bash

set -o errexit -o nounset

BRANCH=$(if [ "$TRAVIS_PULL_REQUEST" == "false" ]; then echo $TRAVIS_BRANCH; else echo $TRAVIS_PULL_REQUEST_BRANCH; fi)

if [ "$BRANCH" == "ci/docs" ]; then
    echo "uploading crate docs"

    pushd json
    cargo doc --no-deps --features "std"
    popd

    cargo doc --no-deps --features "std serde test"

    REV=$(git rev-parse --short HEAD)
    cd target/doc
    git init
    git remote add upstream "https://$GH_TOKEN@github.com/sval-rs/sval"
    git config user.name "sval"
    git config user.email "travis@sval.rs"
    git add -A .
    git commit -qm "Build docs at ${TRAVIS_REPO_SLUG}@${REV}"

    echo "Pushing gh-pages to GitHub"
    git push -q upstream HEAD:refs/heads/gh-pages --force
fi
