#!/usr/bin/env python3
"""
Create simple test MIDI files for testing the parser
"""
import struct

def write_variable_length(value):
    """Convert value to MIDI variable length quantity"""
    result = bytearray()
    result.insert(0, value & 0x7F)
    value >>= 7
    while value > 0:
        result.insert(0, (value & 0x7F) | 0x80)
        value >>= 7
    return bytes(result)

def create_simple_midi():
    """Create a simple MIDI file with a few notes"""
    # Header chunk
    header = b'MThd'
    header += struct.pack('>I', 6)  # Header length
    header += struct.pack('>H', 0)  # Format 0
    header += struct.pack('>H', 1)  # Number of tracks
    header += struct.pack('>H', 480)  # Ticks per quarter note
    
    # Track chunk
    track_events = bytearray()
    
    # Note On: Middle C (60), velocity 100, channel 0, at tick 0
    track_events += write_variable_length(0)  # Delta time
    track_events += bytes([0x90, 60, 100])  # Note On
    
    # Note Off: Middle C (60), channel 0, at tick 480 (1 beat later)
    track_events += write_variable_length(480)  # Delta time
    track_events += bytes([0x80, 60, 0])  # Note Off
    
    # Note On: D (62), velocity 90, channel 0, at same time
    track_events += write_variable_length(0)  # Delta time
    track_events += bytes([0x90, 62, 90])  # Note On
    
    # Note Off: D (62), channel 0, at tick 480 later
    track_events += write_variable_length(480)  # Delta time
    track_events += bytes([0x80, 62, 0])  # Note Off
    
    # End of track
    track_events += write_variable_length(0)  # Delta time
    track_events += bytes([0xFF, 0x2F, 0x00])  # End of track meta event
    
    track = b'MTrk'
    track += struct.pack('>I', len(track_events))
    track += track_events
    
    return header + track

def create_tempo_change_midi():
    """Create a MIDI file with tempo changes"""
    # Header chunk
    header = b'MThd'
    header += struct.pack('>I', 6)
    header += struct.pack('>H', 0)  # Format 0
    header += struct.pack('>H', 1)  # Number of tracks
    header += struct.pack('>H', 480)  # Ticks per quarter note
    
    # Track chunk
    track_events = bytearray()
    
    # Set tempo to 120 BPM at tick 0 (500000 microseconds per quarter note)
    track_events += write_variable_length(0)
    track_events += bytes([0xFF, 0x51, 0x03])  # Tempo meta event
    track_events += struct.pack('>I', 500000)[1:]  # 3 bytes for tempo
    
    # Note On: Middle C
    track_events += write_variable_length(0)
    track_events += bytes([0x90, 60, 100])
    
    # Note Off: Middle C at 480 ticks
    track_events += write_variable_length(480)
    track_events += bytes([0x80, 60, 0])
    
    # Change tempo to 140 BPM (428571 microseconds per quarter note)
    track_events += write_variable_length(0)
    track_events += bytes([0xFF, 0x51, 0x03])
    track_events += struct.pack('>I', 428571)[1:]
    
    # Note On: D
    track_events += write_variable_length(0)
    track_events += bytes([0x90, 62, 90])
    
    # Note Off: D at 480 ticks later
    track_events += write_variable_length(480)
    track_events += bytes([0x80, 62, 0])
    
    # End of track
    track_events += write_variable_length(0)
    track_events += bytes([0xFF, 0x2F, 0x00])
    
    track = b'MTrk'
    track += struct.pack('>I', len(track_events))
    track += track_events
    
    return header + track

def create_multi_track_midi():
    """Create a MIDI file with multiple tracks (Format 1)"""
    # Header chunk
    header = b'MThd'
    header += struct.pack('>I', 6)
    header += struct.pack('>H', 1)  # Format 1
    header += struct.pack('>H', 2)  # Number of tracks
    header += struct.pack('>H', 480)  # Ticks per quarter note
    
    # Track 1: Tempo track
    track1_events = bytearray()
    track1_events += write_variable_length(0)
    track1_events += bytes([0xFF, 0x51, 0x03])
    track1_events += struct.pack('>I', 500000)[1:]
    track1_events += write_variable_length(0)
    track1_events += bytes([0xFF, 0x2F, 0x00])
    
    track1 = b'MTrk'
    track1 += struct.pack('>I', len(track1_events))
    track1 += track1_events
    
    # Track 2: Notes
    track2_events = bytearray()
    track2_events += write_variable_length(0)
    track2_events += bytes([0x90, 60, 100])
    track2_events += write_variable_length(480)
    track2_events += bytes([0x80, 60, 0])
    track2_events += write_variable_length(0)
    track2_events += bytes([0x90, 64, 100])
    track2_events += write_variable_length(480)
    track2_events += bytes([0x80, 64, 0])
    track2_events += write_variable_length(0)
    track2_events += bytes([0xFF, 0x2F, 0x00])
    
    track2 = b'MTrk'
    track2 += struct.pack('>I', len(track2_events))
    track2 += track2_events
    
    return header + track1 + track2

if __name__ == '__main__':
    import os
    
    # Create test_data directory if it doesn't exist
    os.makedirs('tests/test_data', exist_ok=True)
    
    # Create test MIDI files
    with open('tests/test_data/simple_melody.mid', 'wb') as f:
        f.write(create_simple_midi())
    print('Created: tests/test_data/simple_melody.mid')
    
    with open('tests/test_data/tempo_change.mid', 'wb') as f:
        f.write(create_tempo_change_midi())
    print('Created: tests/test_data/tempo_change.mid')
    
    with open('tests/test_data/multi_track.mid', 'wb') as f:
        f.write(create_multi_track_midi())
    print('Created: tests/test_data/multi_track.mid')
