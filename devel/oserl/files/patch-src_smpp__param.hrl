
$FreeBSD: head/devel/oserl/files/patch-src_smpp__param.hrl 362059 2014-07-16 12:26:39Z olgeni $

--- src/smpp_param.hrl.orig
+++ src/smpp_param.hrl
@@ -159,7 +159,7 @@
 %% destination_addr
 %%
 %% %@doc Specifies the destination SME address.  For mobile terminated
-%% messages, this is the directory number of the recipient MS.  IP addresses �
+%% messages, this is the directory number of the recipient MS.  IP addresses
 %% are specified in "aaa.bbb.ccc.ddd" notation.  C-Octet String, Var. max 21
 %% octets.
 %% %@end
