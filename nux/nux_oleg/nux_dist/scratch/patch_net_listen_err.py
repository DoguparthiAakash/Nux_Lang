import re

with open("src/vm.rs", "r") as f:
    content = f.read()

# Add error printing to OP_NET_LISTEN_TLS
net_listen_tls = """                  0xB5 => { // OP_NET_LISTEN_TLS
                       let key_file_id = self.stack.pop().unwrap_or(0);
                       let cert_file_id = self.stack.pop().unwrap_or(0);
                       let port = self.stack.pop().unwrap_or(0) as u16;
                       
                       let mut cert_file = String::new();
                       let mut ptr1 = cert_file_id as usize;
                       loop {
                           let memory = self.shared.memory.read().unwrap();
                           if ptr1 >= memory.len() { break; }
                           let c = memory[ptr1] as char;
                           if c == '\\0' { break; }
                           cert_file.push(c);
                           ptr1 += 1;
                       }
                       
                       let mut key_file = String::new();
                       let mut ptr2 = key_file_id as usize;
                       loop {
                           let memory = self.shared.memory.read().unwrap();
                           if ptr2 >= memory.len() { break; }
                           let c = memory[ptr2] as char;
                           if c == '\\0' { break; }
                           key_file.push(c);
                           ptr2 += 1;
                       }
                       
                       let mut lock = self.shared.listeners.write().unwrap();
                       let id = lock.len();
                       
                       let certs_res = (|| -> Result<Vec<CertificateDer<'static>>, Box<dyn std::error::Error>> {
                           let mut reader = BufReader::new(File::open(&cert_file)?);
                           let certs: Result<Vec<_>, _> = certs(&mut reader).collect();
                           Ok(certs?)
                       })();
                       
                       let key_res = (|| -> Result<PrivateKeyDer<'static>, Box<dyn std::error::Error>> {
                           let mut reader = BufReader::new(File::open(&key_file)?);
                           Ok(private_key(&mut reader)?.ok_or("No private key found")?)
                       })();
                       
                       match (certs_res, key_res) {
                           (Ok(certs_der), Ok(key_der)) => {
                               match ServerConfig::builder().with_no_client_auth().with_single_cert(certs_der, key_der) {
                                   Ok(config) => {
                                       match std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                                           Ok(listener) => {
                                               lock.push(NuxListener::Tls(listener, Arc::new(config)));
                                               self.stack.push(id as i64);
                                           },
                                           Err(e) => {
                                               eprintln!("Failed to bind to port {}: {}", port, e);
                                               self.stack.push(-1);
                                           }
                                       }
                                   },
                                   Err(e) => {
                                       eprintln!("Failed to configure TLS: {}", e);
                                       self.stack.push(-1);
                                   }
                               }
                           },
                           (Err(e1), Err(e2)) => {
                               eprintln!("Failed to read cert file: {}", e1);
                               eprintln!("Failed to read key file: {}", e2);
                               self.stack.push(-1);
                           },
                           (Err(e), _) => {
                               eprintln!("Failed to read cert file: {}", e);
                               self.stack.push(-1);
                           },
                           (_, Err(e)) => {
                               eprintln!("Failed to read key file: {}", e);
                               self.stack.push(-1);
                           }
                       }
                  },"""
content = re.sub(r'0xB5 => \{ // OP_NET_LISTEN_TLS.*?\},\n', net_listen_tls + '\n', content, flags=re.DOTALL)

with open("src/vm.rs", "w") as f:
    f.write(content)
print("Updated OP_NET_LISTEN_TLS with error printing")
