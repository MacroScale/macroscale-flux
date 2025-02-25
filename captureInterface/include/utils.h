#ifndef UTILS_H
#define UTILS_H

#include <string>
#include <unordered_map>
#include <windef.h>

namespace Utils { 
    std::string filepathHWND(HWND hwnd);
    std::unordered_map<HWND, std::string> GetFgWins();
}

#endif
