# ifconfig-rs

[![Production deployed](https://img.shields.io/badge/myip.rs-prod-brightgreen.svg)](http://myip.rs) [![Build Status](https://travis-ci.org/lukaspustina/ifconfig-rs.svg?branch=master)](https://travis-ci.org/lukaspustina/ifconfig-rs) [![GitHub release](https://img.shields.io/github/release/lukaspustina/ifconfig-rs.svg)](https://github.com/lukaspustina/ifconfig-rs/releases) [![license](https://img.shields.io/github/license/lukaspustina/ifconfig-rs.svg)](https://github.com/lukaspustina/ifconfig-rs/blob/master/LICENSE)

_ifconfig-rs_ is yet another <a href="https://www.google.com/search?q=what's+my+ip+address">"what's my IP address"</a> service currently powering [myip.rs](http://myip.rs). It is written in <a href="https://www.rust-lang.org/"> Rust</a> (hence the "-rs" suffix) using the <a href="https://rocket.rs">Rocket</a> web framework and includes GeoLite2 data created by MaxMind, available from <a href="http://www.maxmind.com">http://www.maxmind.com</a>. The UI is made with <a href="https://getuikit.com">uikit</a>. It is MIT licensed so please feel free to clone and to fork it.

_ifconfig_rs_ offers an API to query information like the origin's IP address, TCP port, host name, geoip based location, ISP, as well as user agent. See [myip.rs](http://myip.rs) for API and special CLI tool support.

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->
**Table of Contents**

- [Features](#features)
- [Another "What's my IP" service? But why?](#another-whats-my-ip-service-but-why)
- [Customaziation](#customaziation)
- [Deployment Prerequisites](#deployment-prerequisites)
- [Heroku (and other providers using load balancers)](#heroku-and-other-providers-using-load-balancers)
- [FAQ](#faq)
- [Postcardware](#postcardware)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

## Features

  * It's really fast.

  * Shows your IP address, TCP port, host name, geoip based location, ISP, and user agent.

  * Google Maps integration for geoip location

  * [JSON API](http://myip.rs)

  * Special [support for CLI tools](http://myip.rs) like [curl](https://curl.haxx.se), [httpie](https://github.com/jakubroztocil/httpie), and [wget](https://www.gnu.org/software/wget/). API calls will be answered with just the base information followed by a newline for easy script integration.


## Another "What's my IP" service? But why?

Well, first of all, everybody should have a "What's my IP" service and I wanted to do a little web project using [Rust](https://www.rust-lang.org) and [Rocket](https://rocket.rs). I have been strongly inspired by [ipd](https://github.com/mpolden/ipd) which powers [iconfig.co](http://ifconfig.co). I've added a few details though and I think the UI is nicer.


## Customaziation

The file [Rocket.toml](Rocket.toml) sets the various runtime parameters.

### `runtime_environment`
 If you omit the runtime setting, the TCP connection's remote IP address will be presented as your IP address. In case the runtime is set to "heroku", the `X-Forwarded-To` header is used to determine your IP address. In case you another runtime environment, make sure that you get the real origin IP address -- see section [Heroku](#heroku-and-other-providers-using-load-balancers).

### `project_name`
This sets the title, name etc. of the website in the index.html template file.

### `project_base_url`
The base URL sets the base URL for the CLI examples.

### Web Server Configuration
For web server specific settings like listen IP address and port etc. please see Rocket's [documentation](https://rocket.rs/guide/configuration/#rockettoml).


## Deployment Prerequisites

You have to run `make get_geoip` to retrieve the latest MaxMind geoip and ASN databases.


## Heroku (and other providers using load balancers)

[Heroku](https://heroku.com) uses load balancers to route incoming traffic to apps. On this route, the original remote IP address is masqueraded by the load balancers. Therefore, you can not rely on the TCP connection's remote IP address to identify the request origin's IP address. If the load balancers behave like good citizens, then they add the requesters IP address to the HTTP header `X-Forwarded-For`. From this list, you can gather the origin's IP address. See [Heroku's documentation](https://devcenter.heroku.com/articles/http-routing#heroku-headers) or google for the header field. _ifconfig_rs uses a [Rocket Fairing](https://rocket.rs/guide/fairings/); see [fairings.rs](src/fairings.rs) for details.


## FAQ

  * Where is [myip.rs](http://myip.rs)  hosted?

    The code runs on a free <a href="https://heroku.com">Heroku</a> Autoscaling Dnyo that powers down when there are no service accesses for a period of time. This may lead to a large delay for the first request triggering a service restart.

  * Does _ifconfig-rs_ support IPv6?

    Yes. The code is agnostic regarding the IP version, <a href="https://heroku.com">Heroku</a> currently does not support IPv6, so <em>{{ project.name }}</em> is only available for IPv4. You can run your own instance of _ifconfig-rs_ at another, IPv6 supporting provider.

  * Can I run my own instance of _ifconfig-rs_?

    Yes please, you're welcome to. Just clone or fork this repository.  If you find it useful for your purpose, I would highly appreciate you sending me a postcard from your hometown mentioning how you use it. See my [address](#postcardware).

  * Can you add &lt;feature&gt;, please?

    Yes, why not. Just contact me and let's discuss the details. Better, do it yourself and send me a pull request.

## Postcardware

You're free to use _ifconfig-rs_. If you find it useful, I would highly appreciate you sending me a postcard from your hometown. My work address is

```
Lukas Pustina
CenterDevice GmbH
Rheinwerkallee 3
53227 Bonn
Germany
```

