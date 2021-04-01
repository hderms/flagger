# Flagger
have you ever looked for an easier way to build binary bitmaps? If you're like me, you are still learning to mentally convert from hex to binary, and back, and sometimes all you really want to do is figure out what hex number corresponds to 48 1s in a row. In particular, I found this to cause friction when building https://github.com/hderms/Snowdrift and needing to, for instance, zero out everything past the first 41 binary digits of a number.

## Usage
### Fill

Fill 41 
```
flagger fill -c 41 -r b 
0b11111111111111111111111111111111111111111
```

```
flagger fill -c 41 -r x 
0x1ffffffffff
```