#!/bin/bash

# Make a new repo for the gh-pages branch
rm -rf .gh-pages
mkdir .gh-pages
cd .gh-pages
git init

# Copy over the documentation
cp -r ../target/doc/* .
cp ../circle.yml ./

# Add, commit and push files
git add -f --all .
git commit --author="Joe Wilm <joe@jwilm.com>" -m "Documentation"
git checkout -b gh-pages
git remote add origin https://github.com/jwilm/strava-rs.git
git push -qf origin gh-pages

# Cleanup
cd ..
rm -rf .gh-pages
