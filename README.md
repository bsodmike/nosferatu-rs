# nosferatu

> [!WARNING]  
> This is still a work in progress, and upcoming changes will likely break (a lot!).

## Upcoming

- [x] Panic's can be tested at `/panic`; this renders the panic template in `./templates/panic.html`
- [x] Unhandled routes to render `./templates/error_404.html`
- [ ] TBD

## Get Started

TBD

## Development 

### Migrations

Starting docker will start the Postgres container and create a `./volumes` folder.  The default username and password in the docker `compose` config file is used to create this database the first time it is run.  You can delete the `./volumes` folder, if you run into issues (especially, connecting with the wrong username/password, if in a hurry!).

```
# Start PostgreSQL and other services
just start_docker

just migrate_dev_db
```

## Minimum supported Rust version (MSRV)

This project is tested against rust `stable`.


## License

Licensed under either of [Apache License Version 2.0](./LICENSE-APACHE) or [The MIT License](./LICENSE-MIT) at your option.

🦀 ノ( º \_ º ノ) - respect crables!

## Copyright

Copyright © 2024, [Michael de Silva](mailto:michael@cyberdynea.io)