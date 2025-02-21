#ifndef CAPTURER_H
#define CAPTURER_H

#include <mutex>
#include <winrt/Windows.Graphics.Capture.h>


class Capturer {

public:
    static Capturer& Instance();
    void Init();
    void Capture();

    void StartCapture();
    void EndCapture();
    void ScreenShot();
    void SaveCapture();

private:
    // Static pointer to the Singleton instance
    static Capturer* instancePtr;
    static std::mutex instMutex;

    bool hasInit = false;

    // capture API objects.
    winrt::Windows::Graphics::SizeInt32 lastSize;
    winrt::Windows::Graphics::Capture::GraphicsCaptureItem* item;
    winrt::Windows::Graphics::Capture::Direct3D11CaptureFramePool* framePool;
    winrt::Windows::Graphics::Capture::GraphicsCaptureSession* session;

    Capturer() {
        item = nullptr;
        framePool = nullptr;
        session = nullptr;
    }
    
    // deleting the copy constructor to prevent copies
    Capturer(const Capturer& obj) = delete;
    void operator=(Capturer const&) = delete;
};

static Capturer& CAPTURER = Capturer::Instance();

#endif
