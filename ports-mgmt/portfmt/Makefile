# $FreeBSD$

PORTNAME=	portfmt
PORTVERSION=	g20200206
CATEGORIES=	ports-mgmt

MAINTAINER=	tobik@FreeBSD.org
COMMENT=	Format FreeBSD Ports Makefiles

LICENSE=	BSD2CLAUSE
LICENSE_FILE=	${WRKSRC}/LICENSE

USE_GITHUB=	yes
GH_ACCOUNT=	t6
GH_TAGNAME=	e3d34e1150423e61b9f3e00dd658da21c1584f2d
USE_LDCONFIG=	yes

HAS_CONFIGURE=	yes
CONFIGURE_ARGS=	CPPFLAGS="${CPPFLAGS}" \
		LDFLAGS="${LDFLAGS}" \
		PREFIX="${PREFIX}"
MAKE_ARGS=	INSTALL_LIB="${INSTALL_LIB}" \
		INSTALL_MAN="${INSTALL_MAN}" \
		INSTALL_PROGRAM="${INSTALL_PROGRAM}"
TEST_TARGET=	test

OPTIONS_DEFINE=		SUBPACKAGES

SUBPACKAGES_DESC=	Subpackages support (D16457)

SUBPACKAGES_CPPFLAGS_OFF=	-DPORTFMT_SUBPACKAGES=0

.include <bsd.port.mk>
