# Specification for the format of songs
The song name is taken from the name of the directory containing the song data.
Each directory must include:
- song.wav
- fragments.txt

## `song.wav`
This is a [wav](https://en.wikipedia.org/wiki/WAV) file containing the audio for the song.

## `fragments.txt`
Each line is a "fragment" containing a command and delay.

### Format
delay`:`command

### Delay
The delay is a delay in ms from the start of the song

### Command
| Command | Description   |
| ------- | -----------   |
| L       | New note low  |
| H       | New note high |
| B       | New note both |

### Example
```
30:L
60:H
75:L
```
