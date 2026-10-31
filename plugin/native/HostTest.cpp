#include "Processor.h"
#include <juce_audio_formats/juce_audio_formats.h>
#include <cmath>
#include <cstdlib>
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
struct StateListener final : juce::AudioProcessorListener {
    bool changed = false;
    void audioProcessorParameterChanged(juce::AudioProcessor*, int, float) override {}
    void audioProcessorChanged(juce::AudioProcessor*, const ChangeDetails& details) override {
        changed |= details.nonParameterStateChanged;
    }
};
struct PresetFolder {
    juce::File path = juce::File::getSpecialLocation(juce::File::tempDirectory)
        .getNonexistentChildFile("sad-pet-presets", "", true);
    PresetFolder() {
       #if JUCE_WINDOWS
        _putenv_s("SAD_PET_PRESET_DIR", path.getFullPathName().toRawUTF8());
       #else
        setenv("SAD_PET_PRESET_DIR", path.getFullPathName().toRawUTF8(), 1);
       #endif
    }
    ~PresetFolder() { path.deleteRecursively(); }
};
}

int main() {
    const juce::ScopedJuceInitialiser_GUI juce;
    try {
        PresetFolder presetFolder;
        StateListener listener;
        SadPetProcessor processor;
        processor.addListener(&listener);
        const auto initial = command(processor, "/api/state");
        require(initial["preset"]["name"].toString() == "glasses5", "New instance did not load the startup scene");
        require(initial["file_source"]["sample_name"].toString() == "Dusty keys", "Startup sample was not loaded");
        require(initial["preset_selection"]["id"].toString() == "factory:5", "Startup factory selection is wrong");
        require(!static_cast<bool>(initial["preset_selection"]["dirty"]), "Host normalization marked the startup scene modified");
        require(std::abs(static_cast<float>(initial["preset"]["params"]["filter"]) - 0.4988926f) < 0.00001f,
            "Host parameter defaults replaced the startup filter");
        require(std::abs(processor.parameters.getRawParameterValue("filter")->load() - 0.4988926f) < 0.00001f,
            "Startup values were not published to host automation");
        // Use the neutral DSP for the offset, routing and project-state checks below.
        command(processor, "/api/preset-library/factory", juce::JSON::parse(R"({"index":0})"));
        command(processor, "/api/fades", juce::JSON::parse(R"({"fade_in":0,"fade_out":0})"));
        processor.setPlayConfigDetails(SAD_PET_EFFECT ? 2 : 0, 2, 48000.0, 4096);
        processor.prepareToPlay(48000.0, 4096);
        require(processor.getLatencySamples() == 96, "Master lookahead latency was not reported to the host");
        juce::AudioBuffer<float> buffer(2, 4096);
        juce::MidiBuffer midi;
        buffer.clear();
        render(processor, buffer, midi);
        require(peak(buffer) == 0.0f, "Silence generated nonzero audio");
       #if SAD_PET_EFFECT
        // The granular path needs input history before it can emit sound.
        float inputPeak = 0.0f;
        for (int block = 0; block < 12; ++block) {
            for (int c = 0; c < 2; ++c) for (int i = 0; i < buffer.getNumSamples(); ++i) buffer.setSample(c, i, 0.2f);
            render(processor, buffer, midi);
            inputPeak = juce::jmax(inputPeak, peak(buffer));
        }
        require(inputPeak > 0.01f, "Effect did not process the host input");
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
        require(listener.changed, "Sample import did not notify the DAW to save its changed state");
        auto* position = processor.parameters.getParameter("position");
        position->setValueNotifyingHost(0.15f);
        auto* volume = processor.parameters.getParameter("master_volume");
        volume->setValueNotifyingHost(0.42f);
        command(processor, "/api/parameters", juce::JSON::parse(R"({"position":0.37})"));
        require(std::abs(position->getValue() - 0.37f) < 0.00001f, "Numeric editor change did not publish its host parameter");
        require(std::abs(volume->getValue() - 0.42f) < 0.00001f, "Editor change overwrote a concurrent host automation value");
        auto preset = command(processor, "/api/preset");
        auto pre = juce::JSON::parse(R"([{"id":31,"kind":"waveshaper","mix":0.8,"settings":{"waveshaper_type":"multi_sine","waveshaper_depth":0.7}}])");
        preset.getDynamicObject()->setProperty("pre_color_chain", pre);
        auto source = juce::JSON::parse(R"([{"id":32,"kind":"waveshaper","mix":0.9,"settings":{"waveshaper_type":"hard_clip","waveshaper_depth":0.8}}])");
        preset.getDynamicObject()->setProperty("source_color_chain", source);
        command(processor, "/api/preset", preset);
        command(processor, "/api/parameters", juce::JSON::parse(R"({"pre_fx1_waveshaper_depth":0.33})"));
        require(std::abs(processor.parameters.getParameter("pre_fx1_waveshaper_depth")->getValue() - 0.33f) < 0.00001f,
            "Numeric pre-filter effect edit did not publish its host parameter");
        auto* sourceDepth = processor.parameters.getParameter("source_fx1_waveshaper_depth");
        require(sourceDepth != nullptr, "Source FX host parameter is missing");
        command(processor, "/api/parameters", juce::JSON::parse(R"({"source_fx1_waveshaper_depth":0.61})"));
        require(std::abs(sourceDepth->getValue() - 0.61f) < 0.00001f,
            "Numeric source effect edit did not publish its host parameter");
        auto* region = new juce::DynamicObject;
        region->setProperty("start", 0.25); region->setProperty("end", 0.75);
        command(processor, "/api/region", juce::var(region));
        command(processor, "/api/parameters", juce::JSON::parse(R"({"loop_one_bar":1,"sample_root_note":48})"));
        require(processor.parameters.getParameter("loop_one_bar")->getValue() == 1.0f,
            "Loop checkbox did not publish host automation");
        const auto userPreset = command(processor, "/api/preset-library/save", juce::JSON::parse(R"({"name":"Native patch"})"));
        const auto patchId = userPreset["id"].toString();
        require(presetFolder.path.getChildFile(patchId + ".sadpet").existsAsFile(), "Preset was not saved to disk");
        position->setValueNotifyingHost(0.82f);
        require(static_cast<bool>(command(processor, "/api/state")["preset_selection"]["dirty"]), "Host automation did not mark the patch modified");
        auto* loadPatch = new juce::DynamicObject;
        loadPatch->setProperty("id", patchId);
        listener.changed = false;
        command(processor, "/api/preset-library/load", juce::var(loadPatch));
        require(std::abs(position->getValue() - 0.37f) < 0.00001f, "Preset recall did not publish host automation values");
        require(listener.changed, "Preset recall did not mark the DAW project changed");
        juce::MemoryBlock saved;
        processor.getStateInformation(saved);
        require(saved.getSize() > 100, "Host state was empty");

        SadPetProcessor restored;
        restored.setStateInformation(saved.getData(), static_cast<int>(saved.getSize()));
        auto state = command(restored, "/api/state");
        require(state["preset_selection"]["id"].toString() == patchId, "DAW recall lost the user preset selection");
        require(!static_cast<bool>(state["preset_selection"]["dirty"]), "Clean preset became modified after DAW recall");
        require(state["sample_name"].toString() == "Embedded test.wav", "Custom sample was not restored");
        require(std::abs(static_cast<float>(state["preset"]["params"]["position"]) - 0.37f) < 0.00001f, "Automation value was not restored");
        require(static_cast<double>(state["region"]["start"]) == 0.25, "Sample selection was not restored");
        require(static_cast<bool>(state["preset"]["params"]["loop_one_bar"]), "One-bar loop mode was not restored");
        require(static_cast<int>(state["preset"]["params"]["sample_root_note"]) == 48, "Sample root note was not restored");
        auto preState = state["preset"]["pre_color_chain"][0];
        require(preState["settings"]["waveshaper_type"].toString() == "multi_sine", "Pre-filter waveshaper type was not restored");
        require(std::abs(static_cast<float>(preState["settings"]["waveshaper_depth"]) - 0.33f) < 0.00001f, "Pre-filter FX automation was not restored");
        const auto sourceState = state["preset"]["source_color_chain"][0];
        require(sourceState["settings"]["waveshaper_type"].toString() == "hard_clip", "Source waveshaper type was not restored");
        require(std::abs(static_cast<float>(sourceState["settings"]["waveshaper_depth"]) - 0.61f) < 0.00001f, "Source FX automation was not restored");
        require(command(processor, "/api/state")["sample_name"].toString() == "Embedded test.wav", "Instances shared sample state");
        SadPetProcessor fresh;
        require(command(fresh, "/api/preset-library").getArray()->size() == 1, "User preset library was not shared between instances");
        require(command(fresh, "/api/state")["sample_name"].toString() != "Embedded test.wav", "Fresh instance inherited another instance's state");
        // Malformed state must leave the current project state intact.
        const char corrupt[] = "invalid";
        restored.setStateInformation(corrupt, sizeof(corrupt));
        require(command(restored, "/api/state")["sample_name"].toString() == "Embedded test.wav", "Malformed host state replaced valid state");
        restored.prepareToPlay(96000.0, 4096);
        require(restored.getLatencySamples() == 192, "Master latency did not follow the sample rate");
        require(static_cast<int>(command(restored, "/api/state")["sample_rate"]) == 96000, "Host sample rate change was ignored");
        // Wavetable uses the same native editor/host boundary as the other
        // Rack sources, including the appended C/D automation and DAW recall.
        {
            SadPetProcessor wt;
            const auto instruments = command(wt, "/api/wavetables/presets");
            require(instruments.getArray() != nullptr && instruments.getArray()->size() == 80,
                "Wavetable factory instruments are missing from the native bundle");
            auto* selection = new juce::DynamicObject;
            selection->setProperty("id", instruments[0]["id"]);
            const auto document = command(wt, "/api/wavetables/preset", juce::var(selection));
            command(wt, "/api/preset", document["preset"]);
            for (int count = 3; count <= 4; ++count) {
                const auto added = command(wt, "/api/wavetables/layers", juce::JSON::parse(R"({"action":"add","timbre":0})"));
                require(static_cast<int>(added["wavetable"]["count"]) == count, "Native Rack did not add a wave");
            }
            wt.setPlayConfigDetails(SAD_PET_EFFECT ? 2 : 0, 2, 48000.0, 4096);
            wt.prepareToPlay(48000.0, 4096);
            command(wt, "/api/transport", juce::JSON::parse(R"({"playing":true,"midi_gate":true})"));
            juce::MidiBuffer notes;
            notes.addEvent(juce::MidiMessage::noteOn(1, 60, static_cast<juce::uint8>(100)), 128);
            buffer.clear(); render(wt, buffer, notes);
            require(peak(buffer, 128) > 0.001f, "Native four-wave Rack is silent");
            auto* cPosition = wt.parameters.getParameter("wt_c_position");
            auto* dFine = wt.parameters.getParameter("wt_d_fine");
            require(cPosition != nullptr && dFine != nullptr, "C/D host parameters are missing");
            cPosition->setValueNotifyingHost(0.72f);
            dFine->setValueNotifyingHost(0.455f); // -9 cents in the -100..100 range.
            auto* aFmSource = wt.parameters.getParameter("wt_a_fm_source");
            auto* dSyncSource = wt.parameters.getParameter("wt_d_sync_source");
            auto* bFmMode = wt.parameters.getParameter("wt_b_fm_mode");
            require(aFmSource != nullptr && dSyncSource != nullptr && bFmMode != nullptr,
                "Arbitrary FM/sync host parameters are missing");
            aFmSource->setValueNotifyingHost(1.0f); // D -> A.
            dSyncSource->setValueNotifyingHost(1.0f / 3.0f); // B -> D.
            bFmMode->setValueNotifyingHost(1.0f); // PM is available on B too.
            notes.clear(); buffer.clear(); render(wt, buffer, notes);
            auto current = command(wt, "/api/preset");
            require(std::abs(static_cast<float>(current["wavetable"]["oscillators"][3]["fine"]) + 9.0f) < 0.001f,
                "D automation did not reach the Rack");
            juce::MemoryBlock wtSaved; wt.getStateInformation(wtSaved);
            SadPetProcessor wtRecalled;
            wtRecalled.setStateInformation(wtSaved.getData(), static_cast<int>(wtSaved.getSize()));
            const auto recalled = command(wtRecalled, "/api/preset");
            require(static_cast<int>(recalled["wavetable"]["count"]) == 4, "DAW recall lost a wavetable wave");
            require(std::abs(static_cast<float>(recalled["wavetable"]["oscillators"][2]["position"]) - 0.72f) < 0.001f,
                "DAW recall lost C automation");
            require(static_cast<int>(recalled["wavetable"]["oscillators"][0]["fm_source"]) == 3
                && static_cast<int>(recalled["wavetable"]["oscillators"][3]["sync_source"]) == 1
                && static_cast<int>(recalled["wavetable"]["oscillators"][1]["fm_mode"]) == 2,
                "DAW recall lost arbitrary FM/sync routing");
        }
        std::cout << JucePlugin_Name << ": host audio, MIDI offsets, automation, embedded state, instance isolation and sample-rate checks passed\n";
        return 0;
    } catch (const std::exception& error) {
        std::cerr << error.what() << '\n'; return 1;
    }
}
