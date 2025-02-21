#include "capturer.h"

void Capturer::Init(){
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
