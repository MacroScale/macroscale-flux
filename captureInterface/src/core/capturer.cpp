#include "pch.h"
#include "capturer.h"

using namespace winrt;
using namespace Windows;
using namespace Windows::Foundation;
using namespace Windows::System;
using namespace Windows::Graphics;
using namespace Windows::Graphics::Capture;
using namespace Windows::Graphics::DirectX;
using namespace Windows::Graphics::DirectX::Direct3D11;
using namespace Windows::Foundation::Numerics;
using namespace Windows::UI;
using namespace Windows::UI::Composition;


Capturer::Capturer(IDirect3DDevice const& device, GraphicsCaptureItem const& item) {
    this->item = item;
    this->device = device;

    // Set up 
    auto d3dDevice = GetDXGIInterfaceFromObject<ID3D11Device>(this->device);
    d3dDevice->GetImmediateContext(this->d3dContext.put());

    auto size = this->item.Size();

    this->swapChain = CreateDXGISwapChain(
            d3dDevice, 
            static_cast<uint32_t>(size.Width),
            static_cast<uint32_t>(size.Height),
            static_cast<DXGI_FORMAT>(DirectXPixelFormat::B8G8R8A8UIntNormalized), 
            2);

    // Create framepool, define pixel format (DXGI_FORMAT_B8G8R8A8_UNORM), and frame size. 
    this->framePool = Direct3D11CaptureFramePool::Create(
            this->device,
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            2,
            size);
    this->session = this->framePool.CreateCaptureSession(this->item);
    this->lastSize = size;
    this->frameArrived = this->framePool.FrameArrived(auto_revoke, { this, &Capturer::OnFrameArrived });
};

void Capturer::StartCapture(){
    CheckClosed();
    session.StartCapture();
};

// process captured frames
void Capturer::Close()
{
    auto expected = false;
    if (closed.compare_exchange_strong(expected, true))
    {
		frameArrived.revoke();
		framePool.Close();
        session.Close();

        swapChain = nullptr;
        framePool = nullptr;
        session = nullptr;
        item = nullptr;
    }
}

void Capturer::OnFrameArrived(
    Direct3D11CaptureFramePool const& sender,
    winrt::Windows::Foundation::IInspectable const&)
{
    auto newSize = false;

    {
        auto frame = sender.TryGetNextFrame();
		auto frameContentSize = frame.ContentSize();

        if (frameContentSize.Width != lastSize.Width || frameContentSize.Height != lastSize.Height) {
            // The thing we have been capturing has changed size.
            // We need to resize our swap chain first, then blit the pixels.
            // After we do that, retire the frame and then recreate our frame pool.
            newSize = true;
            lastSize = frameContentSize;
            swapChain->ResizeBuffers(
                2, 
				static_cast<uint32_t>(lastSize.Width),
				static_cast<uint32_t>(lastSize.Height),
                static_cast<DXGI_FORMAT>(DirectXPixelFormat::B8G8R8A8UIntNormalized), 
                0);
        }

        {
            auto frameSurface = GetDXGIInterfaceFromObject<ID3D11Texture2D>(frame.Surface());
            
            com_ptr<ID3D11Texture2D> backBuffer;
            check_hresult(swapChain->GetBuffer(0, guid_of<ID3D11Texture2D>(), backBuffer.put_void()));

            d3dContext->CopyResource(backBuffer.get(), frameSurface.get());
        }
    }

    DXGI_PRESENT_PARAMETERS presentParameters = { 0 };
    swapChain->Present1(1, 0, &presentParameters);

    if (newSize)
    {
        framePool.Recreate(
            device,
            DirectXPixelFormat::B8G8R8A8UIntNormalized,
            2,
            lastSize);
    }
}


void Capturer::ScreenShot(){};
void Capturer::SaveCapture(){};
