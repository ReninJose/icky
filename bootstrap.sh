#!/bin/bash
set -x
set -e

sudo dnf install git rpm-build python3 python3-jinja2 python3-yaml -y
icky_temp=`mktemp -d`
pushd $icky_temp
git clone https://github.com/dbenoit17/icky.git
pushd icky

(sudo ./icky install . -y)
popd;popd
rm -rf $icky_temp

