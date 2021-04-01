# Flagger
have you ever looked for an easier way to build binary bitmaps? If you're like me, you are still learning to mentally convert from hex to binary, and back, and sometimes all you really want to do is figure out what hex number corresponds to 48 1s in a row. In particular, I found this to cause friction when building https://github.com/hderms/Snowdrift and needing to, for instance, zero out everything past the first 41 binary digits of a number.

## Usage
### Fill

Fill a number of binary digits (-c) with 1s and return the output formatted in hex or binary (-b)
```
flagger fill -c 41 -r b 
0b11111111111111111111111111111111111111111
```

```
flagger fill -c 41 -r x 
0x1ffffffffff
```

### Set

Set a single binary digit at index (-c) with output formatted in hex or binary (-b)
```
flagger set -c 41 -r b 
0b10000000000000000000000000000000000000000
```

```
flagger fill -c 41 -r x 
0x10000000000
```


### Invert
Invert a single binary digit at index (-c), with a particular width (-w),  with output formatted in hex or binary (-b)
```
flagger invert -c 12 -r b -w 41 
0b11111111111111111111111111111011111111111
```

```
flagger fill -c 12 -w 41 -r x 
0x1fffffff7ff
```