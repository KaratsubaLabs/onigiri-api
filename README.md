
# onigiri-api

central api for a bunch of services

- anime api using  [Jikan](https://jikan.moe/)
- english dictionary using the [Free Dictionary API](https://dictionaryapi.dev/)
- weather with [wttr.in](https://wttr.in)

## RUNNING FOR DEVELOPEMENT

you can use `entr` to watch the project for changes and recompile:
```
$ find src -type f | entr -r cargo run -j8
```
