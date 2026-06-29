import sys

with open('lib/lumina.nux', 'r') as f:
    code = f.read()

code = code.replace('client.init(client_id);', 'client.init(client, client_id);')
code = code.replace('client.read();', 'client.read(client);')
code = code.replace('client.write(res);', 'client.write(client, res);')
code = code.replace('client.close();', 'client.close(client);')

with open('lib/lumina.nux', 'w') as f:
    f.write(code)

print("Fixed handle_request method calls in lib/lumina.nux")
