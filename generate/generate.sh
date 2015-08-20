#!/bin/sh
set -e

exclude=(XAddPixel
         XDestroyImage
         XGetPixel
         XkbTranslateKey
         XPutPixel
         XSubImage)

contains() {
  local e
  for e in "${@:2}"; do [[ "$e" == "$1" ]] && return 0; done
  return 1
}

generate() {
  local f
  local src="../x11/src/libs/$1.rs"
  local dlsrc="../x11-dl/src/libs/$1.rs"

  echo -e "\e[1;32m-- generate bindings for \"$1\"\e[0m"
  funcs=`readelf -s --wide "$2" | grep FUNC | grep -Eo ' X[A-Za-z0-9]+$' | sed 's/ //' | sort`

  echo "preparing static generator for $1..."
  echo "#include \"generate.h\"" > generate.cpp
  echo "int main() {" >> generate.cpp
  echo "std::cout << \"use libc::*;\" << std::endl;" >> generate.cpp
  echo "std::cout << \"use headers::*;\" << std::endl;" >> generate.cpp
  echo "std::cout << \"extern {\" << std::endl;" >> generate.cpp
  for f in $funcs; do
    if ! contains "$f" "${exclude[@]}"; then
      echo "declare_static(&$f, \"$f\");" >> generate.cpp
    fi
  done
  echo "std::cout << \"}\" << std::endl;" >> generate.cpp
  echo "return 0; }" >> generate.cpp

  echo "compiling static generator for $1..."
  g++ -o generate -std=c++11 -Wno-deprecated-declarations generate.cpp `pkg-config --libs "$1"`

  echo "generating static bindings for $1..."
  ./generate > "../x11/src/libs/$1.rs"
}

generate x11 /usr/lib/libX11.so.6

rm generate generate.cpp
