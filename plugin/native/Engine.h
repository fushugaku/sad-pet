#pragma once
#include <cstddef>
#include <cstdint>

// Audio and Controller are separate opaque owners. Controller calls are serialized
// off the audio thread. Audio is accessed only by the host render callback.
extern "C" {
struct SadController;
struct SadAudio;
struct SadBytes { std::uint8_t* data; std::size_t len; };
// One MIDI clock message at its frame in the block (the app's clock out).
struct SadClockEvent { std::uint32_t offset; std::uint8_t size; std::uint8_t bytes[3]; };
SadBytes sad_parameters();
SadController* sad_control_new(bool effect);
SadController* sad_control_new_standalone();
void sad_control_free(SadController*);
SadAudio* sad_control_prepare(SadController*, std::uint32_t rate);
void sad_audio_free(SadAudio*);
void sad_bytes_free(std::uint8_t*, std::size_t);
void sad_control_set_parameters(SadController*, const float*, std::size_t);
void sad_control_get_parameters(SadController*, float*, std::size_t);
SadBytes sad_control_request(SadController*, const std::uint8_t*, std::size_t);
SadBytes sad_control_save(SadController*); // zlib stream of the scene
bool sad_control_restore(SadController*, const std::uint8_t*, std::size_t);
void sad_audio_begin(SadAudio*, const float*, std::size_t, float bpm, double beat,
                     bool hasBeat, bool playing, std::size_t frames);
void sad_audio_time_signature(SadAudio*, std::uint32_t numerator, std::uint32_t denominator);
// The host renders faster than real time (a bounce): Vocal timbres wait for their phrases.
void sad_audio_offline(SadAudio*, bool offline);
void sad_audio_midi(SadAudio*, const std::uint8_t*, std::size_t);
void sad_audio_process(SadAudio*, const float* left, const float* right,
                      float* outputLeft, float* outputRight, std::size_t frames);
void sad_audio_end(SadAudio*, std::size_t frames);
// The app's MIDI clock out for the block just ended, valid until the next one.
const SadClockEvent* sad_audio_clock_out(const SadAudio*, std::size_t* count);
}
