#include "application_data.h"
#include <cstddef>


void AppData::Init(){
};

AppData& AppData::Instance(){
    static AppData inst;
    if (inst.hasInit == false){
        inst.Init();
        inst.hasInit = true;
    }
    return inst; 
};
