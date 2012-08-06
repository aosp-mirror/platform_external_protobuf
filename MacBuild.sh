#!/bin/sh

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
../../configure --enable-static --disable-shared --target=powerpc-apple CFLAGS="-mmacosx-version-min=10.5 -arch ppc" CXXFLAGS="-mmacosx-version-min=10.5 -arch ppc" LDFLAGS="-mmacosx-version-min=10.5 -arch ppc" --prefix=`pwd`/../../install/ppc 
make
make install
cd ../i386
../../configure --enable-static --disable-shared --target=i386-apple CFLAGS="-mmacosx-version-min=10.5 -arch i386" CXXFLAGS="-mmacosx-version-min=10.5 -arch i386" LDFLAGS="-mmacosx-version-min=10.5 -arch i386" --prefix=`pwd`/../../install/i386
make
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

