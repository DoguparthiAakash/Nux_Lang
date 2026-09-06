import re
import sys

with open("src/vm.rs", "r") as f:
    content = f.read()

# Add imports
imports = """use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::{HashMap, BTreeMap};
use rustls::ServerConfig;
use rustls_pki_types::{CertificateDer, PrivateKeyDer};
use rustls_pemfile::{certs, private_key};
use std::fs::File;
use std::io::BufReader;

pub enum NuxStream {
    Tcp(TcpStream),
    Tls(rustls::StreamOwned<rustls::ServerConnection, TcpStream>),
}

impl Read for NuxStream {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        match self {
            NuxStream::Tcp(s) => s.read(buf),
            NuxStream::Tls(s) => s.read(buf),
        }
    }
}

impl Write for NuxStream {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        match self {
            NuxStream::Tcp(s) => s.write(buf),
            NuxStream::Tls(s) => s.write(buf),
        }
    }
    fn flush(&mut self) -> io::Result<()> {
        match self {
            NuxStream::Tcp(s) => s.flush(),
            NuxStream::Tls(s) => s.flush(),
        }
    }
}

pub enum NuxListener {
    Tcp(TcpListener),
    Tls(TcpListener, Arc<ServerConfig>),
}

impl NuxListener {
    pub fn accept(&self) -> io::Result<NuxStream> {
        match self {
            NuxListener::Tcp(l) => {
                let (stream, _) = l.accept()?;
                Ok(NuxStream::Tcp(stream))
            },
            NuxListener::Tls(l, config) => {
                let (stream, _) = l.accept()?;
                let conn = rustls::ServerConnection::new(Arc::clone(config)).map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
                let tls_stream = rustls::StreamOwned::new(conn, stream);
                Ok(NuxStream::Tls(tls_stream))
            }
        }
    }
}
"""

content = re.sub(r'use std::io::\{.*?\};\n', imports, content, count=1)

# Modify SharedVmState
content = content.replace("pub listeners: RwLock<Vec<std::net::TcpListener>>,", "pub listeners: RwLock<Vec<NuxListener>>,")
content = content.replace("pub connections: RwLock<Vec<std::net::TcpStream>>,", "pub connections: RwLock<Vec<NuxStream>>,")

# Modify OP_NET_LISTEN
net_listen = """                  0xB0 => { // OP_NET_LISTEN
                      let port = self.stack.pop().unwrap_or(0) as u16;
                      let mut lock = self.shared.listeners.write().unwrap();
                      let id = lock.len();
                      if let Ok(listener) = std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                          lock.push(NuxListener::Tcp(listener));
                          self.stack.push(id as i64);
                      } else {
                          self.stack.push(-1);
                      }
                  },"""
content = re.sub(r'0xB0 => \{.*?\},\n', net_listen + '\n', content, flags=re.DOTALL)

# Modify OP_NET_LISTEN_TLS
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
                       
                       if let (Ok(certs_der), Ok(key_der)) = (certs_res, key_res) {
                           if let Ok(config) = ServerConfig::builder().with_no_client_auth().with_single_cert(certs_der, key_der) {
                               if let Ok(listener) = std::net::TcpListener::bind(format!("0.0.0.0:{}", port)) {
                                   lock.push(NuxListener::Tls(listener, Arc::new(config)));
                                   self.stack.push(id as i64);
                               } else {
                                   self.stack.push(-1);
                               }
                           } else {
                               self.stack.push(-1);
                           }
                       } else {
                           self.stack.push(-1);
                       }
                  },"""
content = re.sub(r'0xB5 => \{ // OP_NET_LISTEN_TLS.*?\},\n', net_listen_tls + '\n', content, flags=re.DOTALL)

with open("src/vm.rs", "w") as f:
    f.write(content)
print("Updated vm.rs for TLS")
