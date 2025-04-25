import http.client

conn = http.client.HTTPConnection("0.0.0.0", 47099)
payload = "helloworld/helloworld-parking-stop"
headers = {
  'Content-Type': 'text/plain'
}
conn.request("POST", "/scenario", payload, headers)
res = conn.getresponse()
data = res.read()
print(data.decode("utf-8"))
