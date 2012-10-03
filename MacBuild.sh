#!/bin/sh

set -e 
# Check if we need to build
if [ -f install/protoc ]
then
	echo "Nothing to build in protobuf. Delete install/protoc to force a rebuild"
	exit 0
fi

# Build both PPC and i386 targets
rm -rf build install
mkdir -p build/ppc build/i386 install
cd build/ppc
# Use --host=ppc to keep configure from trying to run a generated ppc file, which will fail.
../../configure --enable-static --disable-shared --target=powerpc-apple CXX=$DEVELOPER_BIN_DIR/g++ CC=$DEVELOPER_BIN_DIR/gcc CFLAGS="-mmacosx-version-min=10.5 -arch ppc -isysroot $SDKROOT" CXXFLAGS="-mmacosx-version-min=10.5 -arch ppc -isysroot $SDKROOT" LDFLAGS="-mmacosx-version-min=10.5 -arch ppc -isysroot $SDKROOT" --prefix=`pwd`/../../install/ppc --host=ppc
# Build with -i (ignore errors) because the build tries to run unit tests.  10.7+
# can't run PPC binaries any longer.
make -j16 -i
make install -i
cd ../i386
../../configure --enable-static --disable-shared --target=i386-apple CXX=$DEVELOPER_BIN_DIR/g++ CC=$DEVELOPER_BIN_DIR/gcc CFLAGS="-mmacosx-version-min=10.5 -arch i386 -isysroot $SDKROOT" CXXFLAGS="-mmacosx-version-min=10.5 -arch i386 -isysroot $SDKROOT" LDFLAGS="-mmacosx-version-min=10.5 -arch i386 -isysroot $SDKROOT" --prefix=`pwd`/../../install/i386
make -j16
make install
cd ../..

# Now merge i386 and PPC
lipo -create ./install/ppc/bin/powerpc-apple-protoc ./install/i386/bin/i386-apple-protoc -output ./install/protoc
lipo -create ./install/ppc/lib/libprotobuf-lite.a ./install/i386/lib/libprotobuf-lite.a -output ./install/libprotobuf-lite.a
lipo -create ./install/ppc/lib/libprotobuf.a ./install/i386/lib/libprotobuf.a -output ./install/libprotobuf.a
lipo -create ./install/ppc/lib/libprotoc.a ./install/i386/lib/libprotoc.a -output ./install/libprotoc.a

# Headers are the same for i386 and ppc
ln -s i386/include install/include

# clean up
git checkout -- Makefile.in
git checkout -- aclocal.m4
git checkout -- configure
git checkout -- src/Makefile.in

