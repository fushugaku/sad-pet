#include "Processor.h"
#include <juce_audio_formats/juce_audio_formats.h>
#include <cmath>
#include <iostream>
#include <stdexcept>

namespace {
void require(bool condition, const char* message) { if (!condition) throw std::runtime_error(message); }
juce::var command(SadPetProcessor& processor, const juce::String& path, const juce::var& data = {}) {
    auto* request = new juce::DynamicObject;
    request->setProperty("path", path);
    request->setProperty("method", data.isVoid() ? "GET" : "POST");
    request->setProperty("data", data);
    const auto response = processor.request(juce::var(request));
    require(static_cast<int>(response["status"]) == 200, "Native editor request failed");
    return response["data"];
}
float peak(const juce::AudioBuffer<float>& buffer, int start = 0, int count = -1) {
    return buffer.getMagnitude(0, start, count < 0 ? buffer.getNumSamples() - start : count);
}
void render(SadPetProcessor& processor, juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midi) { processor.processBlock(buffer, midi); }
}

int main() {
    const juce::ScopedJuceInitialiser_GUI juce;
    try {
        SadPetProcessor processor;
        processor.setPlayConfigDetails(SAD_PET_EFFECT ? 2 : 0, 2, 48000.0, 4096);
        processor.prepareToPlay(48000.0, 4096);
        juce::AudioBuffer<float> buffer(2, 4096);
        juce::MidiBuffer midi;
        buffer.clear();
        render(processor, buffer, midi);
        require(peak(buffer) == 0.0f, "Silence generated nonzero audio");
       #if SAD_PET_EFFECT
        for (int c = 0; c < 2; ++c) for (int i = 0; i < buffer.getNumSamples(); ++i) buffer.setSample(c, i, 0.2f);
        render(processor, buffer, midi);
        require(peak(buffer) > 0.01f, "Effect did not process the host input");
       #else
        midi.addEvent(juce::MidiMessage::noteOn(1, 60, static_cast<juce::uint8>(110)), 512);
        buffer.clear(); render(processor, buffer, midi);
        require(peak(buffer, 0, 512) == 0.0f, "MIDI note played before its sample offset");
        require(peak(buffer, 512) > 0.001f, "Instrument did not render MIDI");
       #endif

        // Custom samples are embedded in DAW state, independent of browser storage.
        juce::MemoryBlock sampleBytes;
        {
            juce::WavAudioFormat wav;
            std::unique_ptr<juce::OutputStream> stream = std::make_unique<juce::MemoryOutputStream>(sampleBytes, false);
            auto writer = wav.createWriterFor(stream, juce::AudioFormatWriterOptions{}
                .withSampleRate(48000).withNumChannels(2).withBitsPerSample(24));
            require(writer != nullptr, "Cannot encode test sample");
            juce::AudioBuffer<float> sample(2, 512);
            for (int c = 0; c < 2; ++c) for (int i = 0; i < sample.getNumSamples(); ++i) sample.setSample(c, i, 0.3f * std::sin(static_cast<float>(i) * 0.13f));
            require(writer->writeFromAudioSampleBuffer(sample, 0, 512), "Cannot write test sample");
        }
        auto* import = new juce::DynamicObject;
        import->setProperty("base64", juce::Base64::toBase64(sampleBytes.getData(), sampleBytes.getSize()));
        import->setProperty("name", "Embedded test.wav");
        import->setProperty("id", "test:embedded");
        command(processor, "/api/wav", juce::var(import));
        auto* position = processor.parameters.getParameter("position");
        position->setValueNotifyingHost(0.37f);
        auto* region = new juce::DynamicObject;
        region->setProperty("start", 0.25); region->setProperty("end", 0.75);
        command(processor, "/api/region", juce::var(region));
        juce::MemoryBlock saved;
        processor.getStateInformation(saved);
        require(saved.getSize() > 100, "Host state was empty");

        SadPetProcessor restored;
        restored.setStateInformation(saved.getData(), static_cast<int>(saved.getSize()));
        auto state = command(restored, "/api/state");
        require(state["sample_name"].toString() == "Embedded test.wav", "Custom sample was not restored");
        require(std::abs(static_cast<float>(state["preset"]["params"]["position"]) - 0.37f) < 0.00001f, "Automation value was not restored");
        require(static_cast<double>(state["region"]["start"]) == 0.25, "Sample selection was not restored");
        require(command(processor, "/api/state")["sample_name"].toString() == "Embedded test.wav", "Instances shared sample state");
        SadPetProcessor fresh;
        require(command(fresh, "/api/state")["sample_name"].toString() != "Embedded test.wav", "Fresh instance inherited another instance's state");
        // Malformed state must leave the current project state intact.
        const char corrupt[] = "invalid";
        restored.setStateInformation(corrupt, sizeof(corrupt));
        require(command(restored, "/api/state")["sample_name"].toString() == "Embedded test.wav", "Malformed host state replaced valid state");
        restored.prepareToPlay(96000.0, 4096);
        require(static_cast<int>(command(restored, "/api/state")["sample_rate"]) == 96000, "Host sample rate change was ignored");
        std::cout << JucePlugin_Name << ": host audio, MIDI offsets, automation, embedded state, instance isolation and sample-rate checks passed\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n'; return 1;
    }
}
