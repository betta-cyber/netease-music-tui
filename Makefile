PREFIX?= /usr/local
BINDIR?= ${PREFIX}/bin
INSTALL?= install
INSTALLDIR= ${INSTALL} -d
INSTALLBIN= ${INSTALL} -m 755

all: ncmt

uninstall:
		rm -f ${DESTDIR}${BINDIR}/ncmt

install:
		${INSTALLDIR} ${DESTDIR}${BINDIR}
		${INSTALLBIN} target/release/ncmt ${DESTDIR}${BINDIR}

ncmt:
	cargo build --release

.PHONY: all install uninstall ncmt
