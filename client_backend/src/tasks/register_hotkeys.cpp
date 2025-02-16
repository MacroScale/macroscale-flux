#include "logger.h"
#include "tasks.h"
#include <windows.h>

void Tasks::RegisterHotKeys::execute(){
    SLOG.info("registering hotkeys");
    SLOG.info("quit hotkey: ALT + Q");
    RegisterHotKey(NULL, 1, MOD_ALT, 0x61); 
}
