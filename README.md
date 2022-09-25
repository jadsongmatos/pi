# pi

https://storage.googleapis.com/pi100t/index.html

https://github.com/Mysticial/DigitViewer

```
Base 10 64-bit integer words with 19 digits per word.
each 8-byte word is little-endian.

decode_word_c_19(const uint8_t *buf, uint64_t *val)
{
    uint64_t v = 0;
    int i;
    for (i = 0; i < 19; i++) {
        *val = (v << 8) | buf[i];
    }
    
}
```