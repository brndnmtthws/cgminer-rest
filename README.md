[![Build Status](https://travis-ci.org/brndnmtthws/cgminer-rest.svg?branch=master)](https://travis-ci.org/brndnmtthws/cgminer-rest) [![Current Crates.io Version](https://img.shields.io/crates/v/cgminer-rest.svg)](https://crates.io/crates/cgminer-rest) [![Coverage Status](https://coveralls.io/repos/github/brndnmtthws/cgminer-rest/badge.svg?branch=master)](https://coveralls.io/github/brndnmtthws/cgminer-rest?branch=master)

# cgminer-rest: a RESTful HTTP wrapper for the cgminer API

![Demo](/demo.gif?raw=true)

This package provides a RESTful HTTP wrapper around the
[cgminer](https://github.com/ckolivas/cgminer) API. Cgminer has a somewhat
esoteric API, which isn't easy to use with standard HTTP-based tooling. Using
this tool, it's easy to interact with cgminer programmatically or using a UI,
such as through a web browser.

## Installing

The package can be installed from [crates.io](https://crates.io/) using the `cargo` tool:

```ShellSession
$ cargo install cgminer-rest
...
$ cgminer-rest
```

See [`Rocket.toml`](Rocket.toml) for configuration details. Instructions on using `Rocket.toml` [can be found here](https://rocket.rs/v0.4/guide/configuration/#rockettoml).

*Note: currently you must be running nightly rust. It is recommended that you use [rustup](https://github.com/rust-lang/rustup.rs) to manage your rust installation.*

## Example

## API

### Endpoints

    🛰  Mounting /:
        => GET /version (version)
        => GET /config (config)
        => GET /summary (summary)
        => GET /devs (devs)
        => GET /devdetails (devdetails)
        => GET /stats (stats)
        => GET /coin (coin)
        => GET /usbstats (usbstats)
        => GET /lcd (lcd)
        => GET /notify (notify)
        => GET /privileged (privileged)
        => PUT /restart (restart)
        => PUT /check/<command> (check)
        => PUT /debug (debug)
        => PUT /hotplug (hotplug)
        => GET /lockstats (lockstats)
        => PUT /zero (zero)
    🛰  Mounting /pools:
        => GET /pools (pools)
        => PUT /pools/<id>/switchto (switchpool)
        => PUT /pools/<id>/enable (enablepool)
        => PUT /pools/<id>/disable (disablepool)
        => POST /pools (addpool)
        => DELETE /pools/<id> (removepool)
        => PUT /pools/<id>/quota (poolquota)
    🛰  Mounting /pga:
        => GET /pga/<id> (pga)
        => GET /pga/count (pgacount)
        => PUT /pga/<id>/enable (pgaenable)
        => PUT /pga/<id>/disable (pgadisable)
        => GET /pga/<id>/identify (pgaidentify)
        => PUT /pga/<id> (pgaset)
    🛰  Mounting /asc:
        => GET /asc/<id> (asc)
        => GET /asc/count (asccount)
        => PUT /asc/<id>/enable (ascenable)
        => PUT /asc/<id>/disable (ascdisable)
        => GET /asc/<id>/identify (ascidentify)
        => PUT /asc/<id> (ascset)

## Project Goals

The goal of this project is to create a high quality API for working with
cgminer-based ASICs, in order to enable development of better tooling for
Bitcoin mining (such as [dragon-rest](https://github.com/brndnmtthws/dragon-rest) and [mother-of-dragons](https://github.com/brndnmtthws/mother-of-dragons)).

If ASIC vendors adopt one canonical API for mining hardware, it will become
much easier to build tooling that works well with many different types of
mining hardware. Nearly all ASIC vendors already use cgminer underneath,
however the cgminer API is esoteric and somewhat difficult to work with
compared to HTTP.
