The MP3 import fixtures are original generated tones, encoded with FFmpeg/libmp3lame.
They contain ID3 and Xing/LAME gapless metadata.

- `mono-44100.mp3`: 440 Hz, 44.1 kHz mono, exactly 16,384 source frames, 96 kbps CBR.
- `stereo-32000.mp3`: 440 Hz left / 660 Hz right, 32 kHz stereo, exactly 8,000 source frames, VBR quality 4.

They verify decoding without changing the sample rate, channel count or duration,
and cover periodic wavetable import and Wave Maker conversion.
