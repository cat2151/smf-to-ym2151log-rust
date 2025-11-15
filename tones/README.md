# YM2151 Tone Definitions

This directory contains tone/voice definitions for the YM2151 FM synthesizer chip.

## File Naming Convention

Tone files should be named according to the MIDI program number they represent:
- `000.json` - MIDI Program 0 (Acoustic Grand Piano)
- `001.json` - MIDI Program 1 (Bright Acoustic Piano)
- ...
- `127.json` - MIDI Program 127 (Gunshot)

## JSON Format

Each tone file contains a JSON object with an `events` array. Each event represents a YM2151 register write:

```json
{
  "events": [
    {
      "time": 0,
      "addr": "0x20",
      "data": "0xC7"
    },
    ...
  ]
}
```

### Event Fields

- `time`: Sample time (usually 0 for tone definitions, as the actual time is set when applying the tone)
- `addr`: Register address in hex format (e.g., "0x20")
- `data`: Data value in hex format (e.g., "0xC7")

### Register Addressing for Channel 0

Tone definitions should be written as if configuring **channel 0**. The converter will automatically adjust the addresses for other channels.

#### Channel-Specific Registers (0x20-0x3F)
- `0x20` - RL_FB_CONNECT: Stereo output, feedback, and algorithm
- `0x38` - PMS/AMS: Phase and amplitude modulation sensitivity

#### Operator-Specific Registers (0x40-0xFF)
Operators are numbered 0-3, with slots calculated as `channel + (operator * 8)`.

For channel 0:
- Operator 0 uses slot 0
- Operator 1 uses slot 8 (0x08)
- Operator 2 uses slot 16 (0x10)
- Operator 3 uses slot 24 (0x18)

Common operator registers:
- `0x40 + slot` - DT1/MUL: Detune and frequency multiplier
- `0x60 + slot` - TL: Total Level (volume)
- `0x80 + slot` - KS/AR: Key Scale and Attack Rate
- `0xA0 + slot` - AMS/D1R: AMS enable and first decay rate
- `0xC0 + slot` - DT2/D2R: Second detune and second decay rate
- `0xE0 + slot` - D1L/RR: First decay level and release rate

### Example: Operator 0, Channel 0

```json
{
  "time": 0,
  "addr": "0x40",  // DT1/MUL for operator 0, channel 0
  "data": "0x01"
}
```

When this tone is applied to channel 3, the converter automatically adjusts it to:
```json
{
  "time": <current_time>,
  "addr": "0x43",  // DT1/MUL for operator 0, channel 3
  "data": "0x01"
}
```

### Registers NOT to Include

Do not include the following in tone definitions, as they are set dynamically during playback:
- `0x08` - Key ON/OFF
- `0x28-0x2F` - KC: Key Code (note pitch)
- `0x30-0x37` - KF: Key Fraction (fine pitch)

## Usage

When a MIDI file contains a Program Change event (e.g., Program Change to 42), the converter will:

1. Look for `tones/042.json` in the current directory
2. If found, load and apply the tone to the channel
3. If not found, use the built-in default tone

## Creating Custom Tones

To create a custom tone:

1. Use the example `000.json` as a template
2. Modify register values according to the [YM2151 datasheet](http://www.appleoldies.ca/ymdatasheet/ym2151.pdf)
3. Save as `{program:03}.json` (e.g., `042.json` for program 42)
4. Test with a MIDI file that uses the corresponding program number

## Built-in Default Tone

If a tone file is not found, the converter uses a simple built-in tone with:
- Algorithm 7 (all operators in parallel)
- Operator 0 as carrier (TL = 0x00)
- Operators 1-3 silent (TL = 0x7F)
- Fast attack (AR = 0x1F)
- Moderate decay (D1R = D2R = 0x05)
- Fast release (RR = 0x07)
