# $FreeBSD$

PORTNAME=	hpipm
DISTVERSION=	0.1.1
CATEGORIES=	math

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	High-performance interior-point-method QP solvers

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

BUILD_DEPENDS=	blasfeo>0:math/blasfeo

USES=		cmake localbase
USE_GITHUB=	yes
GH_ACCOUNT=	giaf

.include <bsd.port.pre.mk>

.if ${ARCH} != x86_64 && ${ARCH} != i386
CMAKE_ARGS+=	-DAVX:BOOL=OFF
.endif

.include <bsd.port.post.mk>
