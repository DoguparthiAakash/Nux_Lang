import re

with open("src/vm.rs", "r") as f:
    content = f.read()

# Add shutdown to NuxStream
nux_stream_shutdown = """impl NuxStream {
    pub fn shutdown(&self, how: std::net::Shutdown) -> io::Result<()> {
        match self {
            NuxStream::Tcp(s) => s.shutdown(how),
            NuxStream::Tls(s) => s.get_ref().shutdown(how),
        }
    }
}
"""

content = content.replace("impl Read for NuxStream {", nux_stream_shutdown + "\nimpl Read for NuxStream {")

with open("src/vm.rs", "w") as f:
    f.write(content)
print("Added shutdown to NuxStream")
