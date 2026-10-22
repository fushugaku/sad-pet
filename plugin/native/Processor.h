#pragma once
#include <juce_audio_processors/juce_audio_processors.h>
#include <juce_gui_extra/juce_gui_extra.h>
#include "Engine.h"
#if SAD_PET_STANDALONE
#include "Standalone.h"
#endif

class SadPetProcessor final : public juce::AudioProcessor {
public:
    SadPetProcessor();
    ~SadPetProcessor() override;
    void prepareToPlay(double, int) override;
    void releaseResources() override {}
    bool isBusesLayoutSupported(const BusesLayout&) const override;
    void processBlock(juce::AudioBuffer<float>&, juce::MidiBuffer&) override;
    juce::AudioProcessorEditor* createEditor() override;
    bool hasEditor() const override { return true; }
    const juce::String getName() const override { return JucePlugin_Name; }
    bool acceptsMidi() const override { return true; }
    bool producesMidi() const override { return false; }
    double getTailLengthSeconds() const override { return 30.0; }
    int getNumPrograms() override { return 1; }
    int getCurrentProgram() override { return 0; }
    void setCurrentProgram(int) override {}
    const juce::String getProgramName(int) override { return {}; }
    void changeProgramName(int, const juce::String&) override {}
    void getStateInformation(juce::MemoryBlock&) override;
    void setStateInformation(const void*, int) override;
    void requestAsync(juce::var request, juce::WebBrowserComponent::NativeFunctionCompletion);
    /// Runs a request off the message thread, serialized with the others.
    void enqueue(juce::var request, juce::WebBrowserComponent::NativeFunctionCompletion);
    /// Waits for queued requests so their completions reach a live editor.
    void finishRequests();
    juce::var request(const juce::var&);

    juce::AudioProcessorValueTreeState parameters;
#if SAD_PET_STANDALONE
    std::unique_ptr<sadpet::Services> standalone;
#endif

private:
    static juce::AudioProcessorValueTreeState::ParameterLayout makeLayout();
    void readParameters(std::vector<float>&) const;
    void publishParameters(const std::vector<float>& previous, bool notifyHost);
    SadController* control = nullptr;
    SadAudio* audio = nullptr;
    std::vector<std::atomic<float>*> rawParameters;
    std::vector<juce::RangedAudioParameter*> hostParameters;
    std::vector<float> renderValues, controlValues;
    juce::CriticalSection controlLock;
    juce::ThreadPool requests { 1 };
    JUCE_DECLARE_NON_COPYABLE_WITH_LEAK_DETECTOR(SadPetProcessor)
};
