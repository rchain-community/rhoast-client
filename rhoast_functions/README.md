### Doc

* Supply `-- --select cli` to run as cli
* Supply `-- --select http` to start http server

1. To run simple deploy rholanmg generator

```
cargo run -- --select cli --utility simple_deploy --simpledeploy valentine

```

2. To run as http, supply `HOST` and `PORT` in `.env` file