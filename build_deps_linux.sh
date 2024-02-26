rm -rf libheif
git clone https://github.com/strukturag/libheif.git
# git clone --branch v1.17.4 https://github.com/strukturag/libheif.git
cd libheif
mkdir build
cd build
cmake --preset=release-noplugins ..
make
cd ../../

export PKG_CONFIG_PATH=`pwd`/libheif/build
#cargo build --release --features heif
#rm -rf libheif
