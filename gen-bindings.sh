#!/bin/sh

bindgen minhook/include/MinHook.h \
	--whitelist-function 'MH_.*' \
	--no-prepend-enum-name \
	-- \
	--target=i686-pc-windows-gnu \
	-I/usr/i686-w64-mingw32/sys-root/mingw/include
