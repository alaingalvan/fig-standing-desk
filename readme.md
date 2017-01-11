# Fig Table

[![License][license-img]][license-url]
[![Donate][donate-img]][donate-url]

A repository with everything involved in the design and engineering of the Fig Table. This includes:

- **Server** - A Rust HTTP server 
- **Client** - A Node.js CLI tool that you can easily send requests to the table from.
- **CAD** - 3D model scematics for the outer table design.

| Product | Store | Cost |
|---------|-------|------|
| Raspberry Pi 3 B+ w/ PSU | [Amazon](https://www.amazon.com/gp/product/B01C6FFNY4/ref=oh_aui_search_detailpage?ie=UTF8&psc=1) | $46.99 |
| 32 GB SD Card | [Amazon](https://www.amazon.com/gp/product/B010Q57T02/ref=od_aui_detailpages00?ie=UTF8&psc=1) | $10.59 |
| Motor Controller | [Ebay](http://www.ebay.com/itm/262745386098?_trksid=p2060353.m2749.l2649&ssPageName=STRK%3AMEBIDX%3AIT) | $14.14 |
| Female to Female GPIO Cables (40) | [Amazon](https://www.amazon.com/gp/product/B00KOL5BCC/ref=oh_aui_detailpage_o09_s00?ie=UTF8&psc=1) | $4.99 |
| DC Power Pigtails (10) | [Amazon](https://www.amazon.com/gp/product/B00CUKHN0S/ref=oh_aui_detailpage_o00_s00?ie=UTF8&psc=1) | $5.28 |
| 12V 10A Power Supply | [Amazon](https://www.amazon.com/gp/product/B00Z9X4GLW/ref=oh_aui_detailpage_o00_s01?ie=UTF8&psc=1) | $16.99 |
| 2x Linear Actuators | [Ebay](http://www.ebay.com/itm/122042491329?_trksid=p2060353.m2749.l2649&ssPageName=STRK%3AMEBIDX%3AIT) | $94.27 |
| Galvanized Steel Pipes | -- | -- |
| Wooden Top | -- | -- |
| Total | w/o tax | **$193.25** | 

## Server

The application that runs on the Raspberry Pi 3 B+, creates an HTTP server at port `74613` (l33t for **table**) that a client can POST to.

- `/api` - POST API endpoint, you send the following schema.

```ts
type APIRequest = {
  // float that describes direction and speed of table.
  vector: number
  // milliseconds to perform action
  time: number
}
```

### Compiling

```bash
rustup target add aarch64-unknown-linux-gnu
cargo run --target=aarch64-unknown-linux-gnu
```

## Client

A CLI application that's meant to be installed as a global module on node. 

```bash
npm i git://github.com/alaingalvan/fig-table#some-branch::/client -g
```

```bash
fig-table Node.js CLI

Usage:
    fig-table <number>
    fig-table [options]

Options:
    -h, --help          Display this message
    -V, --version       Print version info and exit
    -v, --verbose ...   Use verbose output
    -q, --quiet         No output printed to stdout
    -c <ip-address>     Configure the app with a unique ip.
```

## CAD

Blender files used to design and CNC the table.

[license-img]: http://img.shields.io/:license-mit-blue.svg?style=flat-square
[license-url]: https://opensource.org/licenses/MIT
[donate-img]: https://img.shields.io/badge/$-support-green.svg?style=flat-square
[donate-url]: https://www.paypal.me/alaingalvan/3