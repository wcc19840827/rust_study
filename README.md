# README

```bash
# run server
RUST_LOG=info cargo run

# client test
curl -X GET http://localhost:8080/
curl http://127.0.0.1:8080/hello/Alice
curl -X GET http://localhost:8080/client-info
curl -X POST -H "Content-Type: application/json" -d '{"name":"Alice","age":30}' http://127.0.0.1:8080/submit
curl -X POST -d "username=alice&password=secret" http://127.0.0.1:8080/login


#测试 API Key
curl -H "X-API-KEY: my_secret_api_key" http://127.0.0.1:8080/protected
#or
curl "http://127.0.0.1:8080/protected?api_key=my_secret_api_key"
```
