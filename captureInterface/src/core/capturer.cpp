#include "capturer.h"

bool Capturer::Init(){
    return winrt::Windows::Graphics::Capture::GraphicsCaptureSession::IsSupported();
};

Capturer& Capturer::Instance(){
    static Capturer inst;
    if (inst.hasInit == false){
        inst.Init();
        inst.hasInit = true;
    }
    return inst; 
};

void Capturer::Capture(){};
void Capturer::StartCapture(){};
void Capturer::EndCapture(){};
void Capturer::ScreenShot(){};
void Capturer::SaveCapture(){};
