# ipmi-fan-control

[![GitHub license](https://img.shields.io/github/license/yinheli/ipmi-fan-control)](https://github.com/yinheli/ipmi-fan-control/blob/master/LICENSE)

A tool to control the fan speed by monitoring the temperature of CPU via IPMI.

## Why

Our Dell R730 server's iDRAC is not works as expected. The fan always run full speed. And is very noisy, We digged but didn't fix out. So I build this for control the fan speed programmatically. And use RUST just for practice. Any contribute is welcome.

## Usage

Download from [release](https://github.com/yinheli/ipmi-fan-control/releases) page (prebuilt binary via github actions), or build from source code.

```bash
cargo build --release
```

Install dependency, install (debian/pve):

```bash
apt install ipmitool
```

use `ipmi-fan-control --help` to see the usage.

```bash
ipmi-fan-control --help
```

```
USAGE:
    ipmi-fan-control [OPTIONS] <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
        --verbose    Verbose output

SUBCOMMANDS:
    auto     Auto adjust fan speed by interval checking CPU temperature
    fixed    Set fixed RPM percentage for fan
    help     Print this message or the help of the given subcommand(s)
    info     Print CPU temperature and fan RPM
```

## Resource

- https://www.intel.com/content/www/us/en/servers/ipmi/ipmi-home.html
- https://github.com/ipmitool/ipmitool
- https://back2basics.io/2020/05/reduce-the-fan-noise-of-the-dell-r720xd-plus-other-12th-gen-servers-with-ipmi/
