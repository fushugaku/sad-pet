#pragma once
#include <juce_audio_formats/juce_audio_formats.h>
#include <juce_gui_extra/juce_gui_extra.h>
#include <optional>

class SadPetProcessor;

// The standalone app wraps the shared processor with native file dialogs, a
// recorder and the audio device. Sample data stays in native code: the editor
// names an action, the user picks a file, and Rust reads or writes it.
namespace sadpet {

/// Provided by the app. The callbacks run on the message thread.
struct Host {
    std::function<void()> showAudioSettings;
    std::function<void(bool)> setInputWanted;
    std::function<void(const juce::String&)> editorMessage;
    /// The current audio device, readable from any thread.
    void setDevice(const juce::var& summary);
    juce::var describe() const;
    std::atomic<double> cpu { 0.0 };
    /// Engine state answers delivered to the editor.
    std::atomic<int> statePolls { 0 };

private:
    juce::CriticalSection lock;
    juce::var device;
};
Host& host();

#if JUCE_MAC
namespace mac {
/// WKWebView receives editing commands through the responder chain; the app
/// has no Edit menu, so the standard shortcuts are routed here.
void installEditShortcuts();
/// Lets dropped files reach the native editor instead of the WebView.
void routeFileDropsToApp(juce::Component& root);
/// Drag types the embedded WebView still claims; zero once drops reach the app.
int webViewDropTypes(juce::Component& root);
}
#endif

class Recorder {
public:
    struct Take {
        int id = 0;
        juce::File file;
        bool input = false;
        double seconds = 0.0;
    };

    ~Recorder();
    juce::var start(bool input, double sampleRate);
    juce::var stop();
    juce::var state() const;
    std::optional<Take> take(int id) const;
    void saved(int id);
    /// Audio thread: the input before processing, or the output after it.
    void write(const float* const* channels, int numFrames, bool inputStage) noexcept;

private:
    void discard();

    juce::TimeSliceThread thread { "SAD PET recorder" };
    juce::CriticalSection lock;
    juce::SpinLock exchange;
    std::unique_ptr<juce::AudioFormatWriter::ThreadedWriter> writer;
    juce::AudioFormatWriter::ThreadedWriter* active = nullptr;
    std::atomic<bool> recordInput { false };
    std::atomic<std::int64_t> frames { 0 };
    double rate = 48000.0;
    int nextId = 0;
    Take current;
    std::optional<Take> finished;
    juce::String error;
};

class Services {
public:
    explicit Services(SadPetProcessor&);
    /// Message thread. Returns true when the request is answered here.
    bool intercept(const juce::var& request, juce::WebBrowserComponent::NativeFunctionCompletion& complete);
    /// Request thread, once the engine has answered.
    void finish(const juce::var& request, juce::var& result);
    /// Files dropped on the window; done runs on the message thread.
    void importDropped(const juce::StringArray& files, std::function<void(juce::var)> done);

    Recorder recorder;

private:
    void choose(const juce::var& request, juce::WebBrowserComponent::NativeFunctionCompletion complete);

    SadPetProcessor& processor;
    std::unique_ptr<juce::FileChooser> chooser;
    bool choosing = false;
    juce::File lastFolder;
    std::atomic<bool> inputWanted { false };
};

}
