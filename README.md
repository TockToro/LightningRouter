# Lightning Router

Use your device as a router through USB

## Vision

Your device has mobile data. Your Mac does not.
Why not use your device's mobile data on your Mac?

Right now, this program relays a Wireguard connection
from your Mac to a Wireguard server.

Eventually, this app will have a userspace TCP stack
to proxy connections as a NAT.

## Building/Running

Build the dependencies

```sh
cd LightningRouter
make
```

Build the app with Xcode and install on your device

Build the Mac relay

```sh
cd relay
cargo run --release
```

In Wireguard, set the target address to ``127.0.0.1:3400``
(or the IP of your Mac from another device).

In the iOS app, set the target IP to your Wireguard endpoint, and enable.

Profit.

## Speed

But is it fast? *Oh yeah* it's fast. I get roughly 200mbps,
or about 2/3 of my internet connection speed.
Not too bad for overhead.

## License

Not sure yet
