import ssl
import urllib.request

ctx = ssl.create_default_context()
ctx.check_hostname = False
ctx.verify_mode = ssl.CERT_NONE

try:
    with urllib.request.urlopen('https://localhost:8443', context=ctx) as response:
        html = response.read()
        print("Response:", html.decode('utf-8'))
except Exception as e:
    print("Error:", e)
