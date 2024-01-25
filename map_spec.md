# Specification for the format of map files
Each line is a "fragment" containing a command and delay.

## Delay
The delay is a delay in ms starting when the last line's command executes

## Command
| Command | Description   |
| ------- | -----------   |
| L       | New note low  |
| H       | New note high |
| B       | New note both |

## Format
delay`:`command

## Example
```
30:L
30:H
30:L
```
