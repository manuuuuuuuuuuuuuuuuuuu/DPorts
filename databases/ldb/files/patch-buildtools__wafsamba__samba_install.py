--- ./buildtools/wafsamba/samba_install.py.orig	2013-01-27 11:51:43.000000000 +0000
+++ ./buildtools/wafsamba/samba_install.py	2013-01-29 22:49:14.139878631 +0000
@@ -111,7 +111,7 @@
             inst_name    = bld.make_libname(t.target)
     elif self.vnum:
         vnum_base    = self.vnum.split('.')[0]
-        install_name = bld.make_libname(target_name, version=self.vnum)
+        install_name = bld.make_libname(target_name, version=vnum_base)
         install_link = bld.make_libname(target_name, version=vnum_base)
         inst_name    = bld.make_libname(t.target)
         if not self.private_library:
