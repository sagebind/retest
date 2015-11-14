#!/bin/bash
# mkdeb.sh name version arch control installed-size deb-file-path
set -e

SCRIPT=`readlink -f "$0"`
ROOT=`readlink -f $(dirname $SCRIPT)/../..`
cd $ROOT

NAME="$1"
VERSION="$2"
ARCH="$3"
CONTROL_FILE="$4"
INSTALLED_SIZE=$((5 / 1024))
DEB_PATH="$6"
FILE_MODE=755

if [ $ARCH == 'x86_64' ]; then
    ARCH="amd64"
fi

# Create a temporary root directory
TARGET_ROOT="`mktemp -d`"
chmod $FILE_MODE "$TARGET_ROOT"
TARGET="$TARGET_ROOT/$NAME-$VERSION-$ARCH"

# Make and install to deb prefix
mkdir -m $FILE_MODE -p "$TARGET/usr/bin"
make PREFIX="$TARGET/usr" install

# Create the control file
mkdir -m $FILE_MODE -p "$TARGET/DEBIAN"
sed -e "s/@version/$VERSION/g" \
    -e "s/@arch/$ARCH/g" \
    -e "s/@installedSize/$INSTALLED_SIZE/g" \
    "$CONTROL_FILE" > "$TARGET/DEBIAN/control"

# Copy LICENSE.md to /usr/share/doc/retest/copyright
mkdir -m $FILE_MODE -p "$TARGET/usr/share/doc/$NAME"
cp "LICENSE" "$TARGET/usr/share/doc/$NAME/copyright"

# Add lintian overrides
mkdir -m $FILE_MODE -p "$TARGET/usr/share/lintian/overrides"
cp "build/debian/lintian-overrides" "$TARGET/usr/share/lintian/overrides/$NAME"

# Build the deb package
fakeroot dpkg-deb -b "$TARGET"
mv "$TARGET_ROOT/$NAME-$VERSION-$ARCH.deb" "$DEB_PATH"
rm -rf "$TARGET_ROOT"

# Test package
lintian "$DEB_PATH/$NAME-$VERSION-$ARCH.deb"
