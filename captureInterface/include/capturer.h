#ifndef CAPTURER_H
#define CAPTURER_H

class Capturer {

public:
    Capturer(winrt::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice const& device,
        winrt::Windows::Graphics::Capture::GraphicsCaptureItem const& item);

    ~Capturer() { Close(); }

    void StartCapture();
    void EndCapture();
    void ScreenShot();
    void SaveCapture();
    void Close();

private:
    winrt::Windows::Graphics::Capture::GraphicsCaptureItem item{ nullptr };
    winrt::Windows::Graphics::Capture::Direct3D11CaptureFramePool framePool{ nullptr };
    winrt::Windows::Graphics::Capture::GraphicsCaptureSession session{ nullptr };
    winrt::Windows::Graphics::SizeInt32 lastSize;

    winrt::Windows::Graphics::DirectX::Direct3D11::IDirect3DDevice device{ nullptr };
    winrt::com_ptr<IDXGISwapChain1> swapChain{ nullptr };
    winrt::com_ptr<ID3D11DeviceContext> d3dContext{ nullptr };

    std::atomic<bool> closed = false;
    winrt::Windows::Graphics::Capture::Direct3D11CaptureFramePool::FrameArrived_revoker frameArrived;


    void OnFrameArrived(
            winrt::Windows::Graphics::Capture::Direct3D11CaptureFramePool const& sender,
            winrt::Windows::Foundation::IInspectable const& args);

    void CheckClosed()
    {
        if (closed.load() == true)
        {
            throw winrt::hresult_error();
        }
    }
};

#endif
