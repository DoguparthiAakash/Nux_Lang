import os

lexer_path = r"E:\nux\Nux_Lang\nux\nux_oleg\nux_dist\src\lexer.rs"

with open(lexer_path, "r", encoding="utf-8") as f:
    content = f.read()

# Add NetListenTls to Token enum
if "NetListenTls" not in content:
    content = content.replace(
        "NetListen, NetAccept, NetRead, NetWrite, NetClose,",
        "NetListen, NetAccept, NetRead, NetWrite, NetClose, NetListenTls,"
    )

# Add keywords to match text.as_str()
keywords = """            "net_listen" => Token::NetListen,
            "net_accept" => Token::NetAccept,
            "net_read" => Token::NetRead,
            "net_write" => Token::NetWrite,
            "net_close" => Token::NetClose,
            "net_listen_tls" => Token::NetListenTls,
"""

if "net_listen" not in content:
    content = content.replace(
        "\"img_alloc\" => Token::ImgAlloc,",
        keywords + "            \"img_alloc\" => Token::ImgAlloc,"
    )

with open(lexer_path, "w", encoding="utf-8") as f:
    f.write(content)

print("Lexer patched with missing net keywords!")
