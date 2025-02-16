#include "logger.h"
#include "tasks.h"
#include <windows.h>

Tasks::RegisterHotKeys::RegisterHotKeys() { 
    this->SetName("RegisterHotKeys"); 
}

void Tasks::RegisterHotKeys::Execute(){

    SLOG.info("registered quit hotkey: ALT + Q");
    RegisterHotKey(NULL, 1, MOD_ALT, 0x61); 
}
