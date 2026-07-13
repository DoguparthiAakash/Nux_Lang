# FreeBSD Port for Nux Language
# Place in: /usr/ports/lang/nux/
# Build with: cd /usr/ports/lang/nux && make install clean

PORTNAME=       nux
DISTVERSION=    0.4.0
CATEGORIES=     lang
MASTER_SITES=   https://github.com/DoguparthiAakash/Nux_Lang/archive/refs/tags/
DISTNAME=       v${DISTVERSION}

MAINTAINER=     nux@nuxlang.dev
COMMENT=        Universal cross-hardware systems programming language
WWW=            https://github.com/DoguparthiAakash/Nux_Lang

LICENSE=        MIT
LICENSE_FILE=   ${WRKSRC}/LICENSE

BUILD_DEPENDS=  ${LOCALBASE}/bin/cargo:lang/rust

USES=           cargo

# Source is the entire repo tarball
DIST_SUBDIR=    ${PORTNAME}

CARGO_CARGOTOML=    ${WRKSRC}/nux/nux_oleg/nux_portable/Cargo.toml
CARGO_INSTALL_PATH= nux/nux_oleg/nux_portable

do-install:
	${INSTALL_PROGRAM} ${WRKDIR}/target/release/nux ${STAGEDIR}${PREFIX}/bin/nux

.include <bsd.port.mk>
