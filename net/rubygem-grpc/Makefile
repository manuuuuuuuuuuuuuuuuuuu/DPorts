# $FreeBSD$

PORTNAME=	grpc
PORTVERSION=	1.24.0
CATEGORIES=	net rubygems
MASTER_SITES=	RG

MAINTAINER=	mfechner@FreeBSD.org
COMMENT=	Ruby implementation of gRPC

LICENSE=	BSD3CLAUSE

LIB_DEPENDS=	libcares.so:dns/c-ares \
		libgrpc.so:devel/grpc
RUN_DEPENDS=	ca_root_nss>=0:security/ca_root_nss \
		rubygem-googleapis-common-protos-types>=1.0<2.0:devel/rubygem-googleapis-common-protos-types \
		rubygem-google-protobuf>=3.8<4.0:devel/rubygem-google-protobuf

MAKE_JOBS_UNSAFE=	yes

USES=		gem gmake pkgconfig
USE_RUBY=	yes

post-extract:
	@${RM} -r ${WRKSRC}/include ${WRKSRC}/third_party ${WRKSRC}/src/boringssl \
		${WRKSRC}/src/core ${WRKSRC}/Makefile ${WRKSRC}/.yardopts ${WRKSRC}/etc

post-patch:
	@${REINPLACE_CMD} 's,%%LOCALBASE%%,${LOCALBASE},' ${WRKSRC}/src/ruby/lib/grpc.rb
	@${REINPLACE_CMD} -E '/s\.files = /s;, "(Makefile|etc|include|third_party|src/boringssl|src/core)[^"]*"\.freeze;;g' \
		${WRKSRC}/grpc.gemspec
	@${REINPLACE_CMD} -e '/s\.files = /s;"\.yardopts"\.freeze, ;;' ${WRKSRC}/grpc.gemspec

.include <bsd.port.mk>
