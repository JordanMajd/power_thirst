# README

Tool for recording cycling power meter data over BLE

## Notes

Cycling Cadence Cadence (CSC UUID 0x2A5B)
Cycling Speed and Cadence Service (CSCS, UUID 0x1816) 
- Speed & Cadence

Cycling Power Service (CPS, UUID 0x1818)
- Power, Power Per Pedal, Speed, Cadence, Torque
- [flags uint16] [power uint16] [energy uint16] Little-endian
- [52, 0, 41, 0, 101, 18, 5, 49, 0, 0, 140, 135, 203, 22, 159, 36]

## Resources

- [btleplug](https://github.com/deviceplug/btleplug)
- [Pelomon](https://ihaque.org/posts/2021/01/04/pelomon-part-iv-software/)
- [Power Measurement XML](https://github.com/oesmith/gatt-xml/blob/master/org.bluetooth.characteristic.cycling_power_measurement.xml)
- [BLE UUUID Numbers](https://btprodspecificationrefs.blob.core.windows.net/assigned-values/16-bit%20UUID%20Numbers%20Document.pdf)
- [Rust REPL](https://replit.com/languages/rust)
- [Wattdr](https://github.com/woolfel/wattdr)
