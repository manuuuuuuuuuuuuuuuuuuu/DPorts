# $FreeBSD$

PORTNAME=	opennn
DISTVERSION=	0.1-86
DISTVERSIONSUFFIX=	-g449b346
CATEGORIES=	misc # machine-learning

MAINTAINER=	yuri@FreeBSD.org
COMMENT=	Open neural networks library

LICENSE=	GPLv3
LICENSE_FILE=	${WRKSRC}/LICENSE.txt

USES=		cmake compiler:c++11-lang dos2unix
DOS2UNIX_FILES=	CMakeLists.txt
USE_GITHUB=	yes
GH_ACCOUNT=	Artelnics
GH_PROJECT=	OpenNN
USE_LDCONFIG=	yes

CMAKE_OFF=	OpenNN_BUILD_EXAMPLES OpenNN_BUILD_TESTS

OPTIONS_DEFINE=		MPI OPENMP
OPTIONS_DEFAULT=	OPENMP

MPI_CMAKE_BOOL=		__OPENNN_MPI__
MPI_BROKEN=		https://github.com/Artelnics/OpenNN/issues/86

OPENMP_CMAKE_BOOL=	__OPENNN_OMP__
OPENMP_LIB_DEPENDS=	
OPENMP_USES=		localbase:ldflags

do-install:
	${MKDIR} ${STAGEDIR}${PREFIX}/include/${PORTNAME} && ${INSTALL_DATA} ${WRKSRC}/${PORTNAME}/*.h ${STAGEDIR}${PREFIX}/include/${PORTNAME}
	${INSTALL_DATA} ${BUILD_WRKSRC}/${PORTNAME}/libopennn.so ${STAGEDIR}${PREFIX}/lib
	${STRIP_CMD} ${STAGEDIR}${PREFIX}/lib/libopennn.so

.include <bsd.port.mk>
