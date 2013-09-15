--- ./cmake/FindIconv.cmake.orig	2009-08-27 08:46:40.000000000 +1100
+++ ./cmake/FindIconv.cmake	2009-09-16 09:41:24.000000000 +1100
@@ -13,9 +13,9 @@
   SET(ICONV_FIND_QUIETLY TRUE)
 ENDIF (ICONV_INCLUDE_DIR AND ICONV_LIBRARIES)
 
-FIND_PATH(ICONV_INCLUDE_DIR iconv.h) 
+FIND_PATH(ICONV_INCLUDE_DIR iconv.h PATHS /usr/local/include /usr/include ) 
  
-FIND_LIBRARY(ICONV_LIBRARIES NAMES iconv libiconv libiconv-2 c)
+FIND_LIBRARY(ICONV_LIBRARIES NAMES iconv libiconv libiconv-2 c PATHS /usr/local/lib /usr/lib )
  
 IF(ICONV_INCLUDE_DIR AND ICONV_LIBRARIES) 
    SET(ICONV_FOUND TRUE) 
