# hcc-server

source repository for holycharisma server


## building

build binaries managed by nix and direnv

```
direnv allow
cargo build --release
```

## server tech stack

- [rust](https://www.rust-lang.org/)
    - safe, efficient, & fun programming lang
- [tide](https://docs.rs/tide/0.17.0-beta.1/tide/)
    - request/response library - server routing & middleware
    - https://http-rs.github.io/tide-book/
    - [async_session](https://docs.rs/async-session/latest/async_session/)
        - db session cookies that lets you associate arbitrary json with http sessions
- [async-std](https://book.async.rs/)
    - futures and async/await style extensions to rust stdlib
- [askama](https://djc.github.io/askama/)
    - server side templating via rust compile time macros
    - jinja2 style but strong template types
- [sea-orm](https://www.sea-ql.org/SeaORM/)
    - active-record-like object relation mapper

yet to be added:

- [lettre](https://github.com/lettre/lettre)
    - smtp mailer - old fashioned E-MAIL 
    
## server goals

- host a list of email verified members
- identity and authorization
- provide chat and cozyweb space for authorized members
    - create a space where people can link up (secure direct messaging)
- host and distribute media
    - sometimes behind an authorization gate
    - sometimes live broadcasts
- direct patronage & community governance
    - banking // payment integration software
    - software managed community trust