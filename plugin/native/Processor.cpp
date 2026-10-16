#include "Processor.h"
#include <BinaryData.h>
#include <cmath>

namespace {
juce::String takeString(SadBytes bytes) {
    const auto text = juce::String::fromUTF8(reinterpret_cast<const char*>(bytes.data), static_cast<int>(bytes.len));
    sad_bytes_free(bytes.data, bytes.len);
    return text;
}

const juce::var& descriptors() {
    static const auto value = juce::JSON::parse(takeString(sad_parameters()));
    return value;
}

class PluginBrowser final : public juce::WebBrowserComponent {
public:
    explicit PluginBrowser(SadPetProcessor& processor)
        : WebBrowserComponent(makeOptions(processor)) {}

    bool pageAboutToLoad(const juce::String& url) override {
        return url.startsWith(juce::WebBrowserComponent::getResourceProviderRoot())
            || url == "about:blank" || url.startsWith("blob:");
    }

private:
    static std::optional<Resource> resource(const juce::String& path) {
        auto name = path.upToFirstOccurrenceOf("?", false, false);
        if (name == "/" || name.isEmpty()) name = "/index.html";
        if (name == "/juce.js") name = "/index.js";
        name = name.fromLastOccurrenceOf("/", false, false);
        for (int i = 0; i < BinaryData::namedResourceListSize; ++i) {
            if (name != BinaryData::originalFilenames[i]) continue;
            int size = 0;
            const auto* data = BinaryData::getNamedResource(BinaryData::namedResourceList[i], size);
            if (data == nullptr) return {};
            juce::String mime = "application/octet-stream";
            if (name.endsWith(".html")) mime = "text/html";
            else if (name.endsWith(".js")) mime = "text/javascript";
            else if (name.endsWith(".css")) mime = "text/css";
            const auto* begin = reinterpret_cast<const std::byte*>(data);
            return Resource { std::vector<std::byte>(begin, begin + size), mime };
        }
        return {};
    }

    static Options makeOptions(SadPetProcessor& processor) {
        auto options = Options {}
            .withNativeIntegrationEnabled()
            .withResourceProvider(resource)
            .withNativeFunction("sadRequest", [&processor](const juce::Array<juce::var>& args, NativeFunctionCompletion complete) {
                if (args.isEmpty()) { complete(juce::var()); return; }
                processor.requestAsync(args[0], std::move(complete));
            });
       #if JUCE_WINDOWS
        const auto folder = juce::File::getSpecialLocation(juce::File::userApplicationDataDirectory)
            .getChildFile("SAD PET/WebView2");
        options = options.withBackend(Options::Backend::webview2)
            .withWinWebView2Options(Options::WinWebView2 {}.withUserDataFolder(folder).withStatusBarDisabled());
       #endif
        return options;
    }
};

class SadPetEditor final : public juce::AudioProcessorEditor {
public:
    explicit SadPetEditor(SadPetProcessor& owner)
        : AudioProcessorEditor(owner), browser(owner) {
        setResizable(true, true);
        setResizeLimits(420, 540, 1800, 1500);
        setSize(1120, 850);
        addAndMakeVisible(browser);
        browser.goToURL(juce::WebBrowserComponent::getResourceProviderRoot());
    }
    void resized() override { browser.setBounds(getLocalBounds()); }
    void paint(juce::Graphics& g) override { g.fillAll(juce::Colour(0xff20291e)); }
private:
    PluginBrowser browser;
};
}

juce::AudioProcessorValueTreeState::ParameterLayout SadPetProcessor::makeLayout() {
    juce::AudioProcessorValueTreeState::ParameterLayout layout;
    if (auto* array = descriptors().getArray()) {
        for (const auto& spec : *array) {
            const auto min = static_cast<float>(spec["min"]);
            const auto max = static_cast<float>(spec["max"]);
            auto range = juce::NormalisableRange<float>(min, max);
            if (spec["id"].toString().endsWith("_rate")) range.setSkewForCentre(std::sqrt(min * max));
            layout.add(std::make_unique<juce::AudioParameterFloat>(
                juce::ParameterID(spec["id"].toString(), 1), spec["name"].toString(), range, static_cast<float>(spec["default"])));
        }
    }
    return layout;
}

SadPetProcessor::SadPetProcessor()
    : AudioProcessor(BusesProperties()
       #if SAD_PET_EFFECT
        .withInput("Input", juce::AudioChannelSet::stereo(), true)
       #endif
        .withOutput("Output", juce::AudioChannelSet::stereo(), true)),
      parameters(*this, nullptr, "SAD_PET", makeLayout()), control(sad_control_new(SAD_PET_EFFECT != 0)) {
    if (auto* array = descriptors().getArray()) {
        for (const auto& spec : *array) {
            rawParameters.push_back(parameters.getRawParameterValue(spec["id"].toString()));
            hostParameters.push_back(parameters.getParameter(spec["id"].toString()));
        }
    }
    renderValues.resize(rawParameters.size());
    controlValues.resize(rawParameters.size());
    readParameters(controlValues);
    sad_control_set_parameters(control, controlValues.data(), controlValues.size());
}

SadPetProcessor::~SadPetProcessor() {
    requests.removeAllJobs(true, -1);
    sad_audio_free(audio);
    sad_control_free(control);
}

void SadPetProcessor::prepareToPlay(double rate, int) {
    const juce::ScopedLock lock(controlLock);
    readParameters(controlValues);
    sad_control_set_parameters(control, controlValues.data(), controlValues.size());
    sad_audio_free(audio);
    audio = sad_control_prepare(control, static_cast<std::uint32_t>(std::lround(rate)));
    setLatencySamples(0);
}

bool SadPetProcessor::isBusesLayoutSupported(const BusesLayout& layouts) const {
    if (layouts.getMainOutputChannelSet() != juce::AudioChannelSet::stereo()) return false;
   #if SAD_PET_EFFECT
    return layouts.getMainInputChannelSet() == juce::AudioChannelSet::stereo();
   #else
    return layouts.getMainInputChannelSet().isDisabled();
   #endif
}

void SadPetProcessor::readParameters(std::vector<float>& values) const {
    for (std::size_t i = 0; i < rawParameters.size(); ++i)
        values[i] = rawParameters[i]->load(std::memory_order_relaxed);
}

void SadPetProcessor::processBlock(juce::AudioBuffer<float>& buffer, juce::MidiBuffer& midi) {
    const juce::ScopedNoDenormals noDenormals;
    if (audio == nullptr || buffer.getNumChannels() < 2) { buffer.clear(); midi.clear(); return; }
    readParameters(renderValues);
    float bpm = 0.0f;
    double beat = 0.0;
    bool hasBeat = false, playing = false;
    if (auto* playHead = getPlayHead()) {
        if (const auto position = playHead->getPosition()) {
            if (const auto tempo = position->getBpm()) bpm = static_cast<float>(*tempo);
            if (const auto ppq = position->getPpqPosition()) { beat = *ppq; hasBeat = true; }
            playing = position->getIsPlaying();
        }
    }
    const auto frames = buffer.getNumSamples();
    sad_audio_begin(audio, renderValues.data(), renderValues.size(), bpm, beat, hasBeat, playing, static_cast<std::size_t>(frames));
    auto* left = buffer.getWritePointer(0);
    auto* right = buffer.getWritePointer(1);
    const auto render = [&](int start, int count) {
        if (count <= 0) return;
        sad_audio_process(audio,
           #if SAD_PET_EFFECT
            left + start, right + start,
           #else
            nullptr, nullptr,
           #endif
            left + start, right + start, static_cast<std::size_t>(count));
    };
    int cursor = 0;
    for (const auto metadata : midi) {
        const auto offset = juce::jlimit(cursor, frames, metadata.samplePosition);
        render(cursor, offset - cursor);
        sad_audio_midi(audio, metadata.data, static_cast<std::size_t>(metadata.numBytes));
        cursor = offset;
    }
    render(cursor, frames - cursor);
    sad_audio_end(audio, static_cast<std::size_t>(frames));
    for (int channel = 2; channel < buffer.getNumChannels(); ++channel) buffer.clear(channel, 0, frames);
    midi.clear();
}

juce::AudioProcessorEditor* SadPetProcessor::createEditor() { return new SadPetEditor(*this); }

void SadPetProcessor::publishParameters(const std::vector<float>& previous, bool notifyHost) {
    sad_control_get_parameters(control, controlValues.data(), controlValues.size());
    for (std::size_t i = 0; i < hostParameters.size(); ++i) {
        if (std::abs(previous[i] - controlValues[i]) < 0.000001f) continue;
        auto* parameter = hostParameters[i];
        const auto normalized = parameter->convertTo0to1(controlValues[i]);
        if (notifyHost) { parameter->beginChangeGesture(); parameter->setValueNotifyingHost(normalized); parameter->endChangeGesture(); }
        else parameter->setValueNotifyingHost(normalized);
    }
}

juce::var SadPetProcessor::request(const juce::var& requestValue) {
    const juce::ScopedLock lock(controlLock);
    readParameters(controlValues);
    sad_control_set_parameters(control, controlValues.data(), controlValues.size());
    const auto previous = controlValues;
    const auto requestText = juce::JSON::toString(requestValue, true);
    const auto result = juce::JSON::parse(takeString(sad_control_request(control,
        reinterpret_cast<const std::uint8_t*>(requestText.toRawUTF8()), requestText.getNumBytesAsUTF8())));
    if (requestValue["method"].toString() == "POST" && static_cast<int>(result["status"]) == 200) {
        const auto path = requestValue["path"].toString();
        if (path == "/api/preset" || path == "/api/parameters" || path == "/api/preset-library/load" || path == "/api/preset-library/factory")
            publishParameters(previous, true);
        if (path == "/api/preset" || path == "/api/parameters" || path == "/api/wav" || path == "/api/region"
            || path == "/api/fades" || path == "/api/input" || path == "/api/transport"
            || path == "/api/preset-library/load" || path == "/api/preset-library/factory"
            || path == "/api/preset-library/save" || path == "/api/preset-library/delete")
            updateHostDisplay(juce::AudioProcessorListener::ChangeDetails{}.withNonParameterStateChanged(true));
    }
    return result;
}

void SadPetProcessor::requestAsync(juce::var value, juce::WebBrowserComponent::NativeFunctionCompletion complete) {
    requests.addJob([this, payload = std::move(value), finish = std::move(complete)] { finish(request(payload)); });
}

void SadPetProcessor::getStateInformation(juce::MemoryBlock& destination) {
    const juce::ScopedLock lock(controlLock);
    readParameters(controlValues);
    sad_control_set_parameters(control, controlValues.data(), controlValues.size());
    const auto state = sad_control_save(control);
    if (state.len == 0) return;
    destination.reset();
    {
        juce::MemoryOutputStream output(destination, false);
        juce::GZIPCompressorOutputStream compressed(output, 6);
        compressed.write(state.data, state.len);
    }
    sad_bytes_free(state.data, state.len);
}

void SadPetProcessor::setStateInformation(const void* data, int size) {
    if (data == nullptr || size <= 0) return;
    const juce::ScopedLock lock(controlLock);
    juce::MemoryInputStream input(data, static_cast<std::size_t>(size), false);
    juce::GZIPDecompressorInputStream decompressed(input);
    // Bound decompression before handing state to Rust (including corrupt host data).
    juce::MemoryOutputStream state;
    constexpr std::int64_t limit = 256 * 1024 * 1024;
    if (state.writeFromInputStream(decompressed, limit + 1) > limit) return;
    readParameters(controlValues);
    const auto previous = controlValues;
    if (sad_control_restore(control, static_cast<const std::uint8_t*>(state.getData()), state.getDataSize()))
        publishParameters(previous, false);
}

juce::AudioProcessor* JUCE_CALLTYPE createPluginFilter() { return new SadPetProcessor(); }
