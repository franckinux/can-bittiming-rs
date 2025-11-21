This program computes the value of the BTR register (bit timing register) to be
written in a can device in order to meet the bitrate requirement.

This work has been inspired by the well known to the can bus users
[CAN Bit Time Calculation](http://www.bittiming.can-wiki.info/) Internet page.

Currently, it only supports the bxcan device included in the ST MCUs. If you
want support to other devices, please submit PRs.

If the baud rate is not provided, all the possible values are explored (1000000,
500000, 250000, 125000, 100000, 83333, 50000, 20000, 10000 bps).

Optionally, the result can be exported to JSON format.

# Examples

```
can-bittiming-rs -b 125000 -d bxcan -s 87.5 -f 45000000
baudrate: 125000, nbr_tq: 18, brp: 20, sample point: 88.89%, sample point error: 1.56%, ts1: 15, ts2: 2, btr: 0x001e0013
baudrate: 125000, nbr_tq: 15, brp: 24, sample point: 86.67%, sample point error: 0.96%, ts1: 12, ts2: 2, btr: 0x001b0017
baudrate: 125000, nbr_tq: 12, brp: 30, sample point: 91.67%, sample point error: 4.55%, ts1: 10, ts2: 1, btr: 0x0009001d
baudrate: 125000, nbr_tq: 10, brp: 36, sample point: 90.00%, sample point error: 2.78%, ts1: 8, ts2: 1, btr: 0x00070023
baudrate: 125000, nbr_tq: 9, brp: 40, sample point: 88.89%, sample point error: 1.56%, ts1: 7, ts2: 1, btr: 0x00060027
baudrate: 125000, nbr_tq: 8, brp: 45, sample point: 87.50%, sample point error: 0.00%, ts1: 6, ts2: 1, btr: 0x0005002c
baudrate: 125000, nbr_tq: 6, brp: 60, sample point: 83.33%, sample point error: 5.00%, ts1: 4, ts2: 1, btr: 0x0003003b
```

# Usage

```
Computes the value of BTR (register bit timing register) of a can device.

Usage: can-bittiming-rs [OPTIONS] --device <device> --frequency <frequency> --sample-point <sample-point-position>

Options:
  -d, --device <device>
          The devices the timings can be computed for [possible values: bxcan]
  -f, --frequency <frequency>
          Frequency at the entry of the prescaler (Hz)
  -b, --baudrate <baudrate>
          Bits per second
  -s, --sample-point <sample-point-position>
          Sample point position (%)
  -o, --output-format <output-format>
          Output format [possible values: json]
  -j, --sjw <sjw>
          Sync jump width [default: 1]
  -h, --help
          Print help
  -V, --version
          Print version
```
