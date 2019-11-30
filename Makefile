# x86_64-unknown-linux-gnu
# x86_64-apple-darwin
# x86_64-pc-windows-gnu

BIN_NAME:=prf.exe
CRATE_NAME:=prf
MISC:= README.md LICENSE
DIRNAME:=${CRATE_NAME}_${TARGET}

.PHONY: release_all
release_all:
	rm -rf dist/
	make release TARGET=x86_64-pc-windows-gnu    BIN_NAME=${BIN_NAME}
	make release TARGET=x86_64-apple-darwin      BIN_NAME=${CRATE_NAME}
	make release TARGET=x86_64-unknown-linux-gnu BIN_NAME=${CRATE_NAME}

.PHONY: release
release:
	cross build --target ${TARGET} --release
	mkdir -p ${DIRNAME}
	\
	cp ./target/${TARGET}/release/${BIN_NAME} ${DIRNAME}
	cp ${MISC} ${DIRNAME}
	\
	mkdir -p dist
	tar czf dist/${DIRNAME}.tar.gz ${DIRNAME}
	rm -rf ${DIRNAME}
