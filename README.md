# Einwurf

Accept notes, URLs, etc. with a public, minimal web UI (just a text area
and a submit button) without authentication and post them to another,
likely access-restricted destination of choice.

Use case: You are browsing the web on your phone or work computer and
come across an idea or link you want to engage with later (say, at home
on your private computer).

With **Einwurf**, you can just open the web interface, "drop" those
notes, and know they got appended to some list you can access later.

Currently these destinations are supported:

* [Discord](https://discord.com/): Write to a channel.
* [Mattermost](https://mattermost.com/): Write to a channel.
* [Notion](https://www.notion.so/): Append to a page.


## Usage

An example configuration file with explanations on how to obtain or
choose values is included as ``config_example.toml``.

A configuration file is mandatory to run it:

```sh
$ einwurf --config config.toml
```

It is recommended to run Einwurf on `localhost`/`127.0.0.1` only and
expose it through a reverse proxy that also provides
[TLS](https://en.wikipedia.org/wiki/Transport_Layer_Security).

For example, [nginx](https://nginx.org/) could be set up like this (with
[Certbot](https://certbot.eff.org/) used to manage certificates):

```nginx
server {
    listen 443 ssl;
    server_name einwurf.example;

    location / {
        proxy_pass      http://127.0.0.1:3000/;
        proxy_redirect  off;
    }

    ssl_certificate /etc/letsencrypt/live/einwurf.example/fullchain.pem; # managed by Certbot
    ssl_certificate_key /etc/letsencrypt/live/einwurf.example/privkey.pem; # managed by Certbot
}
```


## License

Einwurf is licensed under the MIT license.


## Copyright

Copyright 2022-2023 Jochen Kupperschmidt
