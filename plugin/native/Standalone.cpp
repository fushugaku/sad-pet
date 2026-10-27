#include "Standalone.h"
#include "Processor.h"

namespace sadpet {
namespace {
juce::var envelope(int status, const juce::var& data) {
    auto* object = new juce::DynamicObject();
    object->setProperty("status", status);
    object->setProperty("data", data);
    return juce::var(object);
}

juce::var property(const juce::Identifier& name, const juce::var& value) {
    auto* object = new juce::DynamicObject();
    object->setProperty(name, value);
    return juce::var(object);
}

juce::var failure(const juce::String& message) { return envelope(400, property("error", message)); }

juce::DynamicObject& dataOf(const juce::var& request) {
    if (!request["data"].isObject())
        request.getDynamicObject()->setProperty("data", juce::var(new juce::DynamicObject()));
    return *request["data"].getDynamicObject();
}
}

Host& host() {
    static Host value;
    return value;
}

void Host::setDevice(const juce::var& summary) {
    const juce::ScopedLock guard(lock);
    device = summary;
}

juce::var Host::describe() const {
    const juce::ScopedLock guard(lock);
    return device;
}

Recorder::~Recorder() {
    {
        const juce::SpinLock::ScopedLockType swap(exchange);
        active = nullptr;
    }
    writer.reset();
    thread.stopThread(2000);
    if (current.file.existsAsFile() && (!finished || finished->file != current.file)) current.file.deleteFile();
    discard();
}

juce::var Recorder::start(bool input, double sampleRate) {
    const juce::ScopedLock guard(lock);
    if (writer != nullptr) return state();
    discard();
    error = {};
    rate = sampleRate > 0.0 ? sampleRate : 48000.0;
    auto file = juce::File::getSpecialLocation(juce::File::tempDirectory)
        .getNonexistentChildFile("SAD PET take", ".wav", false);
    std::unique_ptr<juce::OutputStream> stream = file.createOutputStream();
    std::unique_ptr<juce::AudioFormatWriter> created;
    if (stream != nullptr) {
        created = juce::WavAudioFormat().createWriterFor(stream, juce::AudioFormatWriterOptions {}
            .withSampleRate(rate).withNumChannels(2).withBitsPerSample(24));
    }
    if (created == nullptr) {
        stream.reset();
        file.deleteFile();
        error = "Cannot create the recording file.";
        return state();
    }
    thread.startThread();
    writer = std::make_unique<juce::AudioFormatWriter::ThreadedWriter>(created.release(), thread, 1 << 17);
    frames = 0;
    recordInput = input;
    current = { ++nextId, file, input, 0.0 };
    {
        const juce::SpinLock::ScopedLockType swap(exchange);
        active = writer.get();
    }
    return state();
}

juce::var Recorder::stop() {
    const juce::ScopedLock guard(lock);
    if (writer == nullptr) return state();
    {
        const juce::SpinLock::ScopedLockType swap(exchange);
        active = nullptr;
    }
    // Flushes the queued blocks and closes the WAV file.
    writer.reset();
    current.seconds = static_cast<double>(frames.load()) / rate;
    finished = current;
    return state();
}

juce::var Recorder::state() const {
    const juce::ScopedLock guard(lock);
    const auto recording = writer != nullptr;
    const auto take = recording ? current : finished.value_or(Take {});
    auto* object = new juce::DynamicObject();
    object->setProperty("active", recording);
    object->setProperty("available", !recording && finished.has_value());
    object->setProperty("id", take.id);
    object->setProperty("seconds", recording ? static_cast<double>(frames.load()) / rate : take.seconds);
    object->setProperty("source", take.input ? "input" : "output");
    object->setProperty("error", error.isEmpty() ? juce::var() : juce::var(error));
    return juce::var(object);
}

std::optional<Recorder::Take> Recorder::take(int id) const {
    const juce::ScopedLock guard(lock);
    if (finished && finished->id == id) return finished;
    return std::nullopt;
}

void Recorder::saved(int id) {
    const juce::ScopedLock guard(lock);
    if (finished && finished->id == id) discard();
}

void Recorder::discard() {
    if (finished) finished->file.deleteFile();
    finished.reset();
}

void Recorder::write(const float* const* channels, int numFrames, bool inputStage) noexcept {
    if (inputStage != recordInput.load(std::memory_order_relaxed)) return;
    const juce::SpinLock::ScopedTryLockType guard(exchange);
    if (!guard.isLocked() || active == nullptr) return;
    if (active->write(channels, numFrames)) frames.fetch_add(numFrames, std::memory_order_relaxed);
}

Services::Services(SadPetProcessor& owner) : processor(owner) {}

bool Services::intercept(const juce::var& request, juce::WebBrowserComponent::NativeFunctionCompletion& complete) {
    if (!request.isObject()) return false;
    const auto path = request["path"].toString().upToFirstOccurrenceOf("?", false, false);
    const auto post = request["method"].toString() == "POST";
    // Only a native dialog or the recorder may name a file on disk.
    if (auto* data = request["data"].getDynamicObject()) {
        data->removeProperty("path");
        data->removeProperty("paths");
    }
    if (request["dialog"].isObject()) {
        choose(request, std::move(complete));
        return true;
    }
    if (path == "/api/app/audio-settings") {
        if (host().showAudioSettings) host().showAudioSettings();
        complete(envelope(200, property("ok", true)));
        return true;
    }
    if (path == "/api/app/audio-outputs") {
        complete(host().audioOutputs ? envelope(200, host().audioOutputs()) : failure("No audio device is available."));
        return true;
    }
    if (path == "/api/app/audio-output" && post) {
        const auto error = host().setAudioOutput ? host().setAudioOutput(request["data"]["output"].toString())
                                                 : juce::String("No audio device is available.");
        complete(error.isEmpty() ? envelope(200, property("ok", true)) : failure(error));
        return true;
    }
    if (path == "/api/app/log") {
        const auto message = request["data"]["message"].toString();
        juce::Logger::writeToLog("SAD PET editor: " + message);
        if (host().editorMessage) host().editorMessage(message);
        complete(envelope(200, property("ok", true)));
        return true;
    }
    if (path == "/api/recording" && post) {
        const auto& data = request["data"];
        complete(envelope(200, static_cast<bool>(data["active"])
            ? recorder.start(data["source"].toString() == "input", processor.getSampleRate())
            : recorder.stop()));
        return true;
    }
    if (path == "/api/library/take") {
        const auto id = static_cast<int>(request["data"]["id"]);
        const auto take = recorder.take(id);
        if (!take) {
            complete(failure("The recording is no longer available."));
            return true;
        }
        auto& data = dataOf(request);
        data.setProperty("path", take->file.getFullPathName());
        data.setProperty("source", take->input ? "input" : "output");
        processor.enqueue(request, [this, id, finish = std::move(complete)](juce::var result) {
            if (static_cast<int>(result["status"]) == 200) recorder.saved(id);
            finish(result);
        });
        return true;
    }
    return false;
}

void Services::choose(const juce::var& request, juce::WebBrowserComponent::NativeFunctionCompletion complete) {
    if (choosing) {
        complete(failure("A file dialog is already open."));
        return;
    }
    const auto& dialog = request["dialog"];
    const auto save = dialog["mode"].toString() == "save";
    const auto suggested = juce::File::createLegalFileName(dialog["filename"].toString());
    const auto folder = lastFolder.isDirectory() ? lastFolder
        : juce::File::getSpecialLocation(juce::File::userMusicDirectory);
    chooser = std::make_unique<juce::FileChooser>(dialog["title"].toString(),
        save && suggested.isNotEmpty() ? folder.getChildFile(suggested) : folder,
        dialog["patterns"].toString());
    // A voicebank import may ask for a folder (a UTAU voicebank) instead of a file.
    int flags = static_cast<bool>(dialog["folders"]) ? juce::FileBrowserComponent::canSelectDirectories
                                                     : juce::FileBrowserComponent::canSelectFiles;
    if (save) flags |= juce::FileBrowserComponent::saveMode | juce::FileBrowserComponent::warnAboutOverwriting;
    else flags |= juce::FileBrowserComponent::openMode;
    if (!save && static_cast<bool>(dialog["multiple"])) flags |= juce::FileBrowserComponent::canSelectMultipleItems;
    choosing = true;
    const auto multiple = (flags & juce::FileBrowserComponent::canSelectMultipleItems) != 0;
    const auto extension = suggested.contains(".") ? suggested.fromLastOccurrenceOf(".", true, false) : juce::String();
    chooser->launchAsync(flags, [this, request, save, multiple, extension, finish = std::move(complete)](const juce::FileChooser& fc) {
        choosing = false;
        const auto files = fc.getResults();
        if (files.isEmpty()) {
            finish(envelope(200, property("cancelled", true)));
            return;
        }
        lastFolder = files.getFirst().getParentDirectory();
        auto& data = dataOf(request);
        // A single choice arrives as "path", a multiple selection as "paths".
        if (multiple) {
            juce::Array<juce::var> paths;
            for (const auto& file : files) paths.add(file.getFullPathName());
            data.setProperty("paths", paths);
        } else {
            auto file = files.getFirst();
            if (save && extension.isNotEmpty() && !file.hasFileExtension(extension)) file = file.withFileExtension(extension);
            data.setProperty("path", file.getFullPathName());
        }
        processor.enqueue(request, finish);
    });
}

void Services::importDropped(const juce::StringArray& files, std::function<void(juce::var)> done) {
    auto* request = new juce::DynamicObject();
    request->setProperty("path", "/api/library/import");
    request->setProperty("method", "POST");
    juce::Array<juce::var> paths;
    for (const auto& file : files) paths.add(file);
    request->setProperty("data", property("paths", paths));
    processor.enqueue(juce::var(request), [finish = std::move(done)](juce::var result) {
        juce::MessageManager::callAsync([finish, result] { finish(result); });
    });
}

void Services::finish(const juce::var& request, juce::var& result) {
    const auto path = request["path"].toString().upToFirstOccurrenceOf("?", false, false);
    if (static_cast<int>(result["status"]) != 200) return;
    auto* data = result["data"].getDynamicObject();
    if (data == nullptr) return;
    if (path == "/api/clock") {
        // The engine answers Follow and the lead; the output is the app's.
        if (request["data"].hasProperty("out")) {
            const auto out = request["data"]["out"];
            clockOut.open(out.isVoid() ? juce::String() : out.toString());
        }
        clockOut.listOutputs(*data);
        clockOut.describe(*data);
        return;
    }
    if (path != "/api/state") return;
    if (auto* clock = data->getProperty("clock").getDynamicObject()) clockOut.describe(*clock);
    data->setProperty("recording", recorder.state());
    ++host().statePolls;
    const auto device = host().describe();
    if (device.isObject()) {
        for (const auto& name : { "audio_device", "audio_error", "audio_enabled", "audio_input" })
            data->setProperty(name, device[name]);
        data->setProperty("cpu_percent", host().cpu.load());
    }
    const auto wanted = static_cast<bool>(result["data"]["input"]["active"]);
    if (inputWanted.exchange(wanted) != wanted) {
        juce::MessageManager::callAsync([wanted] {
            if (host().setInputWanted) host().setInputWanted(wanted);
        });
    }
}

ClockOut::ClockOut() : juce::Thread("SAD PET MIDI clock") { startThread(juce::Thread::Priority::high); }

ClockOut::~ClockOut() { stopThread(2000); }

juce::String ClockOut::open(const juce::String& identifier) {
    std::unique_ptr<juce::MidiOutput> opened;
    juce::String name;
    if (identifier.isNotEmpty()) {
        for (const auto& device : juce::MidiOutput::getAvailableDevices())
            if (device.identifier == identifier) name = device.name;
        opened = name.isEmpty() ? nullptr : juce::MidiOutput::openDevice(identifier);
    }
    const juce::ScopedLock guard(lock);
    output = std::move(opened);
    outputId = output != nullptr ? identifier : juce::String();
    outputName = output != nullptr ? name : juce::String();
    error = identifier.isNotEmpty() && output == nullptr ? "Could not open that MIDI output" : "";
    active = output != nullptr;
    return error;
}

void ClockOut::listOutputs(juce::DynamicObject& answer) const {
    juce::Array<juce::var> outputs;
    for (const auto& device : juce::MidiOutput::getAvailableDevices()) {
        auto* port = new juce::DynamicObject();
        port->setProperty("id", device.identifier);
        port->setProperty("name", device.name);
        outputs.add(juce::var(port));
    }
    answer.setProperty("outputs", outputs);
    const juce::ScopedLock guard(lock);
    answer.setProperty("out", outputId.isNotEmpty() ? juce::var(outputId) : juce::var());
}

void ClockOut::describe(juce::DynamicObject& clock) const {
    const juce::ScopedLock guard(lock);
    clock.setProperty("sending", outputName.isNotEmpty() ? juce::var(outputName) : juce::var());
    clock.setProperty("error", error.isNotEmpty() ? juce::var(error) : juce::var());
}

void ClockOut::push(const SadClockEvent* events, std::size_t count, double sampleRate, double latency) noexcept {
    if (events == nullptr || count == 0 || sampleRate <= 0.0 || !active.load(std::memory_order_relaxed)) return;
    const auto start = juce::Time::getMillisecondCounterHiRes() + latency * 1000.0;
    const auto scope = fifo.write(static_cast<int>(juce::jmin(count, static_cast<std::size_t>(fifo.getFreeSpace()))));
    std::size_t index = 0;
    const auto fill = [&](int first, int size) {
        for (int i = first; i < first + size; ++i, ++index) {
            const auto& event = events[index];
            auto& message = queue[static_cast<std::size_t>(i)];
            message.time = start + event.offset * 1000.0 / sampleRate;
            message.size = juce::jmin<std::uint8_t>(event.size, 3);
            std::copy(event.bytes, event.bytes + 3, message.bytes);
        }
    };
    fill(scope.startIndex1, scope.blockSize1);
    fill(scope.startIndex2, scope.blockSize2);
}

void ClockOut::run() {
    while (!threadShouldExit()) {
        int start1 = 0, size1 = 0, start2 = 0, size2 = 0;
        fifo.prepareToRead(1, start1, size1, start2, size2);
        if (size1 == 0) {
            wait(2);
            continue;
        }
        const auto& next = queue[static_cast<std::size_t>(start1)];
        const auto due = next.time - juce::Time::getMillisecondCounterHiRes();
        if (due > 0.5) {
            // The thread wakes to the millisecond: a tick goes out within one.
            wait(juce::jmax(1, static_cast<int>(due)));
            continue;
        }
        {
            const juce::ScopedLock guard(lock);
            if (output != nullptr) output->sendMessageNow(juce::MidiMessage(next.bytes, next.size));
        }
        fifo.finishedRead(1);
    }
}

}
