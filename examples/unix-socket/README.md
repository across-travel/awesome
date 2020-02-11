## Unix domain socket example

```bash
$ curl --unix-socket /tmp/kayrx-uds.socket http://localhost/
Hello world!
```

Although this will only one thread for handling incoming connections 
according to the 
[documentation](https://docs.rs/kayrx/web/struct.HttpServer.html#method.bind_uds).

And it does not delete the socket file (`/tmp/kayrx-uds.socket`) when stopping
the server so it will fail to start next time you run it unless you delete
the socket file manually.
