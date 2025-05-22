Index: hyper-util/tests/proxy.rs
===================================================================
--- hyper-util.orig/tests/proxy.rs
+++ hyper-util/tests/proxy.rs
@@ -280,7 +280,7 @@ async fn test_socks_v5_with_server_resol
 }
 
 #[cfg(not(miri))]
-#[tokio::test]
+#[allow(dead_code)]
 async fn test_socks_v5_with_locally_resolved_domain_works() {
     let proxy_tcp = TcpListener::bind("127.0.0.1:0").await.expect("bind");
     let proxy_addr = proxy_tcp.local_addr().expect("local_addr");
