Bevy App template for ODROID-N2L with ODROID-VU5A
=================================================

The default configuration presumes `clang` and `mold` are installed. It is tuned for development on linux/amd64 and release on-device.
This baseline has been tested with Linux 6.1, Panfrost drivers and Weston.

On-device dependencies
----------------------

```sh
apt install clang mold libasound2-dev libwayland-dev libxkbcommon-dev
```
