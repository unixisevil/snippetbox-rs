# snippetbox-rs -  snippetbox  in rust

I port the demo app "snippet box" in [Let's Go](https://lets-go.alexedwards.net/) to rust using [Actix web framework ](https://github.com/actix) and [Askama template rendering engine](https://github.com/djc/askama/) and [compile-time checked SQLx ](https://github.com/launchbadge/sqlx)


Learning  rust  web programming  from  [Zero To Production In Rust](https://github.com/LukeMathWalker/zero-to-production)   

![shot 1](demo-shot-1.png)

![shot 2](demo-shot-2.png)

![shot 3](demo-shot-3.png)



## Building  && Running



### manual

```bash
./scripts/init_db.sh && ./scripts/init_redis.sh
```

```bash
 cargo run --release
```



### docker-compose

```bash
 docker-compose up -d
```
