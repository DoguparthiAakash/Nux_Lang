#!/usr/bin/env bash
set -e

# Scaffolds and builds an .rpm package for RHEL/Fedora distributions

VERSION="1.0.0"
echo "Building RPM Package..."

mkdir -p rpmbuild/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
cat <<EOF > rpmbuild/SPECS/nux.spec
Name:           nux
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Nux Programming Language

License:        MIT
URL:            https://github.com/DoguparthiAakash/Nux_Lang

%description
An incredibly fast, lightweight, and extensible programming language.

%install
mkdir -p %{buildroot}/usr/local/lib/nux
mkdir -p %{buildroot}/usr/local/bin
# Copy files into buildroot here

%files
/usr/local/lib/nux/*
/usr/local/bin/nux
EOF

if command -v rpmbuild &> /dev/null; then
    rpmbuild -ba rpmbuild/SPECS/nux.spec --define "_topdir $(pwd)/rpmbuild"
    echo "RPM built in rpmbuild/RPMS/"
else
    echo "rpmbuild not found. Run this script on a RHEL/Fedora system."
fi
