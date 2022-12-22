# klepp.me

A service to share clips among friends, while owning and managing our own data

This is an attempted rewrite of [klepp-backend](https://github.com/klepp-me/klepp-backend) in Python to rust.
The Python version is stable and works fine, but is slow and the ffmpeg integration is **hacky**.  

My aim with this project is:

* Learn rust, this is my first rust project
* Have a cleaner ffmpeg integration
* Quicker API

The stack I've decided on before writing any code (so this may change) is:

* [tokio-rs/axum](https://github.com/tokio-rs/axum) as the API framework
  * I'm not sure how to handle auth yet, but I hope I won't have to write it from scratch like I did in Python.
* <not sure about how I'll do the db>
* [zmwangx/rust-ffmpeg](https://github.com/zmwangx/rust-ffmpeg) to process videos and create thumbnails

## Project structure

I'll have a somewhat similar project structure to what I'd like to have in
FastAPI. 

```bash 
├── src
│   ├── api  <-- Actual APIs
│   ├── config.rs  <-- Configuration
│   ├── core.rs  <-- Configuration, such as the serve function
│   ├── core 
│   │   ├── error.rs  <-- exception handler
│   │   ├── extractor.rs  <-- auth in the future?
│   │   ├── profiles.rs
│   │   └── users.rs
│   └── main.rs
```

## Python (FastAPI) vs Rust (Axum), notes to myself

* `mod.rs` was somewhat similar to `__init__.py`. Required to expose a new module named like that folder.
  * In [`rustc` > `1.3`](https://doc.rust-lang.org/reference/items/modules.html#module-source-filenames) this is rather
  done with naming a `<foldername>.rs` file, so in our case we'd have a `core` folder,
  and a `core.rs` file.
    * TODO: investigate difference between `pub mod error;` and `mod error`
* `Extractors` is similar to `ctx` in `arq`, and allow you to share a state in the app. This can be a pool of 
 database connections or a way to validate a JWT header. It's recommended over `extensions` according to the [docs](https://docs.rs/axum/latest/axum/#sharing-state-with-handlers), 
  which is more similar to `Depends` in FastAPI. We'll use `extractors` in this project. 
* 