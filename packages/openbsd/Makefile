DISTNAME=           nux-0.4.0
CATEGORIES=         lang
MASTER_SITES=       https://github.com/DoguparthiAakash/Nux_Lang/archive/refs/tags/
DIST_SUBDIR=        nux

MAINTAINER=         nux@nuxlang.dev
COMMENT=            Universal cross-hardware systems programming language
HOMEPAGE=           https://github.com/DoguparthiAakash/Nux_Lang

# MIT license
PERMIT_PACKAGE=     Yes
PERMIT_DISTFILES=   Yes

MODULES=            lang/rust

BUILD_DEPENDS=      cargo->=1.70:lang/rust>=1.70

MODRUST_PKGDIR=     nux/nux_oleg/nux_portable

do-build:
	cd ${WRKDIST}/nux/nux_oleg/nux_portable && \
	    ${SETENV} ${MODRUST_ENV} \
	    cargo build --release --locked

do-install:
	${INSTALL_PROGRAM} ${WRKDIST}/nux/nux_oleg/nux_portable/target/release/nux \
	    ${PREFIX}/bin/nux

.include <bsd.port.mk>
