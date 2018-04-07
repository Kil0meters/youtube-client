#!/bin/sh

VERSION=$1
DEST=${MESON_BUILD_ROOT}
DIST=$DEST/dist/$VERSION


cd "${MESON_SOURCE_ROOT}"
mkdir -p $DIST

# copying files
cp -rf sentinel-data $DIST
cp -rf sentinel-gtk $DIST
cp -rf sentinel-downloader $DIST
cp Cargo.toml $DIST
cp configure $DIST
cp meson.build $DIST
cp Sentinel.doap $DIST
cp LICENSE $DIST
cp README.md $DIST
cp -rf scripts $DIST

# cargo vendor
mkdir $DIST/.cargo
cargo vendor > $DIST/.cargo/config
cp -rf vendor $DIST/

# packaging
cd $DEST/dist
tar -cJvf $VERSION.tar.xz $VERSION
