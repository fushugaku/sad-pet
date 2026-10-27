#include "Processor.h"
#include <juce_audio_utils/juce_audio_utils.h>
#include <juce_audio_plugin_client/Standalone/juce_StandaloneFilterWindow.h>

// The SAD PET app: the shared processor on the computer's audio and MIDI
// devices, in one native window. JUCE's standalone holder owns the devices;
// this app adds the window, menu, session file and on-demand audio input.
namespace {
class AppMenu final : public juce::MenuBarModel {
public:
    juce::StringArray getMenuBarNames() override { return {}; }
    juce::PopupMenu getMenuForIndex(int, const juce::String&) override { return {}; }
    void menuItemSelected(int id, int) override {
        if (id == 1 && sadpet::host().showAudioSettings) sadpet::host().showAudioSettings();
    }
};

class MainWindow final : public juce::DocumentWindow {
public:
    MainWindow(juce::AudioProcessor& owner, juce::PropertySet& store)
        : DocumentWindow("SAD PET", juce::Colour(0xff20291e), DocumentWindow::allButtons),
          processor(owner), settings(store), editor(owner.createEditorAndMakeActive()) {
        setUsingNativeTitleBar(true);
        setContentNonOwned(editor.get(), true);
        setResizable(true, false);
        setResizeLimits(480, 540, 16384, 16384);
        const auto saved = settings.getValue("windowState");
        if (saved.isEmpty() || !restoreWindowStateFromString(saved)) centreWithSize(getWidth(), getHeight());
        setVisible(true);
       #if JUCE_MAC
        sadpet::mac::routeFileDropsToApp(*editor);
       #endif
    }

    ~MainWindow() override {
        settings.setValue("windowState", getWindowStateAsString());
        clearContentComponent();
        processor.editorBeingDeleted(editor.get());
        editor = nullptr;
    }

    void closeButtonPressed() override { juce::JUCEApplication::getInstance()->systemRequestedQuit(); }

private:
    juce::AudioProcessor& processor;
    juce::PropertySet& settings;
    std::unique_ptr<juce::AudioProcessorEditor> editor;
};

class SadPetApplication final : public juce::JUCEApplication,
                                private juce::ChangeListener,
                                private juce::Timer {
public:
    SadPetApplication() {
        juce::PropertiesFile::Options options;
        options.applicationName = "SAD PET";
        options.filenameSuffix = ".settings";
        // Tests keep the settings and session out of the user's folder.
        const auto folder = juce::SystemStats::getEnvironmentVariable("SAD_PET_APP_DIR", {});
        options.folderName = folder.isNotEmpty() ? folder : juce::String("SAD PET");
        options.osxLibrarySubFolder = "Application Support";
        properties.setStorageParameters(options);
    }

    const juce::String getApplicationName() override { return JucePlugin_Name; }
    const juce::String getApplicationVersion() override { return JucePlugin_VersionString; }
    // A test instance with its own folder may run beside the user's app.
    bool moreThanOneInstanceAllowed() override {
        return juce::SystemStats::getEnvironmentVariable("SAD_PET_APP_DIR", {}).isNotEmpty();
    }
    void anotherInstanceStarted(const juce::String&) override {
        if (window != nullptr) window->toFront(true);
    }

    void initialise(const juce::String&) override {
        auto& settings = *properties.getUserSettings();
        // No input channels at launch: the microphone opens when the editor asks for it.
        juce::Array<juce::StandalonePluginHolder::PluginInOuts> channels;
        channels.add({ 0, 2 });
        holder = std::make_unique<juce::StandalonePluginHolder>(&settings, false, juce::String(), nullptr, channels, true);
        holder->processorHasPotentialFeedbackLoop = false;
        holder->getMuteInputValue().setValue(false);
        restoreSession();

        auto& host = sadpet::host();
        host.showAudioSettings = [this] { holder->showAudioSettingsDialog(); };
        host.setInputWanted = [this](bool wanted) { setInputWanted(wanted); };
        host.editorMessage = [this](const juce::String& message) { editorMessages.add(message); };
        host.audioOutputs = [this] { return audioOutputs(); };
        host.setAudioOutput = [this](const juce::String& name) { return setAudioOutput(name); };
        holder->deviceManager.addChangeListener(this);
        changeListenerCallback(nullptr);

        window = std::make_unique<MainWindow>(*holder->processor, settings);
       #if JUCE_MAC
        juce::PopupMenu extra;
        extra.addItem(1, "Audio/MIDI Settings…");
        juce::MenuBarModel::setMacMainMenu(&menu, &extra);
        sadpet::mac::installEditShortcuts();
       #endif
        smokeReport = juce::SystemStats::getEnvironmentVariable("SAD_PET_SMOKE_TEST", {});
        smokeStarted = juce::Time::getMillisecondCounter();
        startTimer(500);
    }

    void shutdown() override {
        stopTimer();
       #if JUCE_MAC
        juce::MenuBarModel::setMacMainMenu(nullptr);
       #endif
        auto& host = sadpet::host();
        host.showAudioSettings = nullptr;
        host.setInputWanted = nullptr;
        host.editorMessage = nullptr;
        host.audioOutputs = nullptr;
        host.setAudioOutput = nullptr;
        if (holder != nullptr) {
            holder->deviceManager.removeChangeListener(this);
            holder->stopPlaying();
            saveSession();
            if (auto* processor = dynamic_cast<SadPetProcessor*>(holder->processor.get())) processor->finishRequests();
        }
        window = nullptr;
        holder = nullptr;
        properties.saveIfNeeded();
    }

    void systemRequestedQuit() override {
        if (juce::ModalComponentManager::getInstance()->cancelAllModalComponents()) {
            juce::Timer::callAfterDelay(100, [] {
                if (auto* app = juce::JUCEApplicationBase::getInstance()) app->systemRequestedQuit();
            });
        } else {
            quit();
        }
    }

private:
    juce::File sessionFile() { return properties.getUserSettings()->getFile().getSiblingFile("Session.sadstate"); }

    // The whole scene, including its samples, survives a restart of the app.
    void restoreSession() {
        juce::MemoryBlock data;
        if (sessionFile().loadFileAsData(data) && data.getSize() > 0)
            holder->processor->setStateInformation(data.getData(), static_cast<int>(data.getSize()));
    }

    void saveSession() {
        juce::MemoryBlock data;
        holder->processor->getStateInformation(data);
        if (data.getSize() > 0) sessionFile().replaceWithData(data.getData(), data.getSize());
    }

    juce::var audioOutputs() {
        auto& manager = holder->deviceManager;
        juce::Array<juce::var> outputs;
        if (auto* type = manager.getCurrentDeviceTypeObject()) {
            // Rescan so devices plugged in since launch are offered.
            type->scanForDevices();
            for (const auto& name : type->getDeviceNames(false)) outputs.add(name);
        }
        auto* result = new juce::DynamicObject();
        result->setProperty("outputs", outputs);
        result->setProperty("output", manager.getAudioDeviceSetup().outputDeviceName);
        return juce::var(result);
    }

    juce::String setAudioOutput(const juce::String& name) {
        auto& manager = holder->deviceManager;
        auto* type = manager.getCurrentDeviceTypeObject();
        if (type == nullptr || !type->getDeviceNames(false).contains(name))
            return "This audio output is no longer connected.";
        auto setup = manager.getAudioDeviceSetup();
        if (setup.outputDeviceName == name) return {};
        setup.outputDeviceName = name;
        setup.useDefaultOutputChannels = true;
        const auto error = manager.setAudioDeviceSetup(setup, true);
        changeListenerCallback(nullptr);
        return error;
    }

    void setInputWanted(bool wanted) {
        auto& manager = holder->deviceManager;
        auto* type = manager.getCurrentDeviceTypeObject();
        auto setup = manager.getAudioDeviceSetup();
        const auto open = setup.inputDeviceName.isNotEmpty() && setup.inputChannels.countNumberOfSetBits() > 0;
        if (type == nullptr || wanted == open) return;
        auto& settings = *properties.getUserSettings();
        if (wanted) {
            const auto names = type->getDeviceNames(true);
            auto name = settings.getValue("inputDevice");
            if (!names.contains(name)) name = names[juce::jmax(0, type->getDefaultDeviceIndex(true))];
            if (name.isEmpty()) {
                inputError = "No audio input device is available.";
                changeListenerCallback(nullptr);
                return;
            }
            setup.inputDeviceName = name;
            setup.inputChannels.clear();
            setup.inputChannels.setRange(0, 2, true);
        } else {
            // Closing the input also turns off the system's microphone indicator.
            settings.setValue("inputDevice", setup.inputDeviceName);
            setup.inputDeviceName = {};
            setup.inputChannels.clear();
        }
        setup.useDefaultInputChannels = false;
        inputError = manager.setAudioDeviceSetup(setup, true);
        changeListenerCallback(nullptr);
    }

    void changeListenerCallback(juce::ChangeBroadcaster*) override {
        auto* summary = new juce::DynamicObject();
        auto* device = holder->deviceManager.getCurrentAudioDevice();
        const auto inputs = device != nullptr ? device->getActiveInputChannels().countNumberOfSetBits() : 0;
        // MIDI clock out goes when the audio rendered with it is heard.
        sadpet::host().outputLatency = device != nullptr && device->getCurrentSampleRate() > 0.0
            ? (device->getOutputLatencyInSamples() + device->getCurrentBufferSizeSamples()) / device->getCurrentSampleRate()
            : 0.0;
        summary->setProperty("audio_device", device != nullptr ? device->getName() : juce::String("No audio device"));
        summary->setProperty("audio_enabled", device != nullptr);
        summary->setProperty("audio_input", inputs > 0);
        summary->setProperty("audio_error", device == nullptr ? juce::var("Choose an audio output in Audio/MIDI settings.")
            : inputError.isNotEmpty() ? juce::var(inputError) : juce::var());
        sadpet::host().setDevice(juce::var(summary));
    }

    void timerCallback() override {
        sadpet::host().cpu = holder->deviceManager.getCpuUsage() * 100.0;
        if (smokeReport.isNotEmpty()) checkSmokeTest();
    }

    // CI launches the app with SAD_PET_SMOKE_TEST=<report file>: it passes once
    // the editor has loaded and polls the engine without reporting an error.
    void checkSmokeTest() {
        const auto polls = sadpet::host().statePolls.load();
        const auto elapsed = juce::Time::getMillisecondCounter() - smokeStarted;
        auto failed = !editorMessages.isEmpty();
        if (!failed && polls < 25 && elapsed < 60000) return;
        auto* report = new juce::DynamicObject();
       #if JUCE_MAC
        const auto dropTypes = sadpet::mac::webViewDropTypes(*window->getContentComponent());
        report->setProperty("webview_drop_types", dropTypes);
        failed |= dropTypes != 0;
       #endif
        report->setProperty("ok", !failed && polls >= 25);
        report->setProperty("state_polls", polls);
        report->setProperty("editor_messages", editorMessages);
        report->setProperty("audio", sadpet::host().describe());
        juce::File(smokeReport).replaceWithText(juce::JSON::toString(juce::var(report)));
        smokeReport = {};
        setApplicationReturnValue(!failed && polls >= 25 ? 0 : 1);
        systemRequestedQuit();
    }

    juce::ApplicationProperties properties;
    std::unique_ptr<juce::StandalonePluginHolder> holder;
    std::unique_ptr<MainWindow> window;
    AppMenu menu;
    juce::String inputError;
    juce::StringArray editorMessages;
    juce::String smokeReport;
    juce::uint32 smokeStarted = 0;
};
}

juce::JUCEApplicationBase* juce_CreateApplication();
juce::JUCEApplicationBase* juce_CreateApplication() { return new SadPetApplication(); }
