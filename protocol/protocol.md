## Structure
String: `4C5409`
Dev ID: `00`
Dev Type: `30`
The string: `5700`
The command: 2 chars
The string: `01`
The parameter: (2-4 chars)
The 4-chars CRC-16/XMODEM checksum
    - the checksum is computed by taking the concatenated value of the preceding components and interpreting it as hexadecimal byte values

### Example
on: 4C5409 0030 5700 00 01 01 22DF
off:4C5409 0030 5700 00 01 00 32FE

## Commands
  "00" -> turn on/off
        Params:
            "00" - off
            "01" - on
  "02" -> set brightness (parameter is percentage of intensity; for example: 63)
  "03" -> set temperature (parameter is 3200-5600 devided by 100)
  "04" -> set hue (parameter is 0-360 devided by 5)
  "05" -> set saturation (parameter is 0-100)
