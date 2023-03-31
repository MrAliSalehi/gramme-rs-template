# gramme-rs-template
basic ready-to-run template for gramme-rs library 


## how to use


`cargo generate --git https://github.com/MrAliSalehi/gramme-rs-template`

### note

for handling your telegram account you have 2 options:

**1.** create a `config.json` file like [this](/config.json)

**2.** pass your account with CLI arguments:

- `./gramme-rs-template --id 222 --hash xxx --phone +1111`

*you can use `-i` for instead of `--id` and `-a` instead of `--hash`*

after passing the arguments, IF they were valid it automatically save them 
in `config.json` and will re-use them in the next time.



