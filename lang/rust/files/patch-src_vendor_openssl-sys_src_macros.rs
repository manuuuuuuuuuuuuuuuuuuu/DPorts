--- src/vendor/openssl-sys/src/macros.rs.orig	2018-09-16 20:29:19 UTC
+++ src/vendor/openssl-sys/src/macros.rs
@@ -0,0 +1,69 @@
+// vendored from the cfg-if crate to avoid breaking ctest
+macro_rules! cfg_if {
+    // match if/else chains with a final `else`
+    ($(
+        if #[cfg($($meta:meta),*)] { $($it:item)* }
+    ) else * else {
+        $($it2:item)*
+    }) => {
+        cfg_if! {
+            @__items
+            () ;
+            $( ( ($($meta),*) ($($it)*) ), )*
+            ( () ($($it2)*) ),
+        }
+    };
+
+    // match if/else chains lacking a final `else`
+    (
+        if #[cfg($($i_met:meta),*)] { $($i_it:item)* }
+        $(
+            else if #[cfg($($e_met:meta),*)] { $($e_it:item)* }
+        )*
+    ) => {
+        cfg_if! {
+            @__items
+            () ;
+            ( ($($i_met),*) ($($i_it)*) ),
+            $( ( ($($e_met),*) ($($e_it)*) ), )*
+            ( () () ),
+        }
+    };
+
+    // Internal and recursive macro to emit all the items
+    //
+    // Collects all the negated cfgs in a list at the beginning and after the
+    // semicolon is all the remaining items
+    (@__items ($($not:meta,)*) ; ) => {};
+    (@__items ($($not:meta,)*) ; ( ($($m:meta),*) ($($it:item)*) ), $($rest:tt)*) => {
+        // Emit all items within one block, applying an approprate #[cfg]. The
+        // #[cfg] will require all `$m` matchers specified and must also negate
+        // all previous matchers.
+        cfg_if! { @__apply cfg(all($($m,)* not(any($($not),*)))), $($it)* }
+
+        // Recurse to emit all other items in `$rest`, and when we do so add all
+        // our `$m` matchers to the list of `$not` matchers as future emissions
+        // will have to negate everything we just matched as well.
+        cfg_if! { @__items ($($not,)* $($m,)*) ; $($rest)* }
+    };
+
+    // Internal macro to Apply a cfg attribute to a list of items
+    (@__apply $m:meta, $($it:item)*) => {
+        $(#[$m] $it)*
+    };
+}
+
+macro_rules! stack {
+    ($t:ident) => {
+        cfg_if! {
+            if #[cfg(ossl110)] {
+                pub enum $t {}
+            } else {
+                #[repr(C)]
+                pub struct $t {
+                    pub stack: ::_STACK,
+                }
+            }
+        }
+    }
+}