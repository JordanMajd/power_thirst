# README

## BLE



Cycling Power Measurement 00002a63-0000-1000-8000-00805f9b34fb



## Services

Cycling Cadence Cadence (CSC UUID 0x2A5B)
Cycling Speed and Cadence Service (CSCS, UUID 0x1816) 
- Speed & Cadence

Cycling Power Service (CPS, UUID 0x1818)
- Power, Power Per Pedal, Speed, Cadence, Torque
- [flags uint16] [power uint16] [energy uint16] Little-endian
- [52, 0, 41, 0, 101, 18, 5, 49, 0, 0, 140, 135, 203, 22, 159, 36]

## Resources

- [Pelomon](https://ihaque.org/posts/2021/01/04/pelomon-part-iv-software/)
- [BLE UUUID Numbers](https://btprodspecificationrefs.blob.core.windows.net/assigned-values/16-bit%20UUID%20Numbers%20Document.pdf)
