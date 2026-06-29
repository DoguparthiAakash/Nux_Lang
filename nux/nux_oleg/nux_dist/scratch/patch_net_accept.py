import re

with open("src/vm.rs", "r") as f:
    content = f.read()

# Patch OP_NET_ACCEPT
net_accept = """                  0xB1 => { // OP_NET_ACCEPT
                      let id = self.stack.pop().unwrap_or(0) as usize;
                      let lock = self.shared.listeners.read().unwrap();
                      if id < lock.len() {
                          if let Ok(stream) = lock[id].accept() {
                              let mut conn_lock = self.shared.connections.write().unwrap();
                              let conn_id = conn_lock.len();
                              conn_lock.push(stream);
                              self.stack.push(conn_id as i64);
                          } else {
                              self.stack.push(-1);
                          }
                      } else {
                          self.stack.push(-1);
                      }
                  },"""
content = re.sub(r'0xB1 => \{ // OP_NET_ACCEPT.*?\},\n', net_accept + '\n', content, flags=re.DOTALL)

with open("src/vm.rs", "w") as f:
    f.write(content)
print("Updated OP_NET_ACCEPT")
