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

    std::wstring wsPath(path);
    std::string res(wsPath.begin(), wsPath.end());

    delete[] title;
    delete[] path;

    return res;
}


BOOL CALLBACK EnumWindowsProc(HWND hwnd, LPARAM lParam) {
    // skip if window is invisible
    if (!IsWindowVisible(hwnd)) {
        return TRUE;
    }

	char class_name[80];
	char title[80];
	GetClassName(hwnd,class_name, sizeof(class_name));
	GetWindowText(hwnd,title,sizeof(title));

    std::unordered_map<HWND, std::string>* titles = 
        reinterpret_cast<std::unordered_map<HWND, std::string>*>(lParam);

    if (strlen(title) > 0){
        titles->insert({hwnd, title});
    }

	return TRUE;
}

std::unordered_map<HWND, std::string> Utils::GetFgWins(){
    std::unordered_map<HWND, std::string> fgWins;
    EnumWindows(EnumWindowsProc, reinterpret_cast<LPARAM>(&fgWins));
    return fgWins;
}
