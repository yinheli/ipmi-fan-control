# ipmi-fan-control

A tool to control the fan speed by monitoring the temperature of CPU via IPMI.

## why

Our Dell R730 server's iDRAC is not works as expected. The fan always run full speed. And is very noisy, We digged but didn't fix out. So I build this for control the fan speed programmatically. And use RUST just for practice. Any contribute is welcome.

## usage

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

## resource

- https://www.intel.com/content/www/us/en/servers/ipmi/ipmi-home.html
- https://github.com/ipmitool/ipmitool
- https://back2basics.io/2020/05/reduce-the-fan-noise-of-the-dell-r720xd-plus-other-12th-gen-servers-with-ipmi/
