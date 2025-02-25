#include "utils.h"
#include <string>
#include <windows.h>

std::string Utils::filepathHWND(HWND hwnd){
    int lgth = GetWindowTextLength(hwnd) + 1;
    wchar_t* title = new wchar_t[lgth];
    GetWindowTextW(hwnd, title, lgth);

    DWORD id;
    GetWindowThreadProcessId(hwnd, &id); 

    wchar_t* path = new wchar_t[MAX_PATH];
    DWORD size = MAX_PATH;
    HANDLE hProc = OpenProcess(PROCESS_QUERY_INFORMATION, FALSE, id);
    QueryFullProcessImageNameW(hProc, 0, path, &size);
    CloseHandle(hProc);

    std::wstring ws(path);

    return std::string(ws.begin(), ws.end());
}
