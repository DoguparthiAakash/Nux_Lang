#include <windows.h>
#include <commctrl.h>
#include <shellapi.h>
#include <string>
#include <thread>
#include <fstream>
#include <msi.h>
#include "resource.h"

// Link common controls, gdi32, and msi
#pragma comment(lib, "comctl32.lib")
#pragma comment(lib, "gdi32.lib")
#pragma comment(lib, "msi.lib")
#pragma comment(linker,"\"/manifestdependency:type='win32' name='Microsoft.Windows.Common-Controls' version='6.0.0.0' processorArchitecture='*' publicKeyToken='6595b64144ccf1df' language='*'\"")

#define BTN_INSTALL   1001
#define BTN_CANCEL    1002
#define BTN_REPAIR    1003
#define BTN_UNINSTALL 1004
#define BTN_FINISH    1005

enum class SetupState { Welcome, Maintenance, Installing, Complete, Failed };

HWND g_hWnd = NULL;
HWND g_hProgressBar = NULL;
HWND g_hStatusText = NULL;
HWND g_hTitle = NULL;
HWND g_hDesc = NULL;

HWND g_btnInstall = NULL;
HWND g_btnCancel = NULL;
HWND g_btnRepair = NULL;
HWND g_btnUninstall = NULL;
HWND g_btnFinish = NULL;

SetupState g_state = SetupState::Welcome;
HBITMAP g_hLogo = NULL;

// Nux Brand Colors
COLORREF COLOR_BG = RGB(20, 24, 30);       // Dark charcoal
COLORREF COLOR_ACCENT = RGB(0, 191, 255);  // Electric Blue
COLORREF COLOR_TEXT = RGB(255, 255, 255);  // White

HBRUSH hBgBrush = CreateSolidBrush(COLOR_BG);

const char* UPGRADE_CODE = "{A1B2C3D4-E5F6-4789-9A0B-1C2D3E4F5A6B}";

bool IsNuxInstalled() {
    char productCode[39] = {0};
    UINT res = MsiEnumRelatedProductsA(UPGRADE_CODE, 0, 0, productCode);
    return res == ERROR_SUCCESS;
}

void UpdateUIState() {
    ShowWindow(g_btnInstall, SW_HIDE);
    ShowWindow(g_btnCancel, SW_HIDE);
    ShowWindow(g_btnRepair, SW_HIDE);
    ShowWindow(g_btnUninstall, SW_HIDE);
    ShowWindow(g_btnFinish, SW_HIDE);
    ShowWindow(g_hProgressBar, SW_HIDE);
    ShowWindow(g_hStatusText, SW_HIDE);
    
    switch (g_state) {
        case SetupState::Welcome:
            SetWindowTextA(g_hTitle, "Welcome to Nux");
            SetWindowTextA(g_hDesc, "Nux is a universal, cross-hardware systems programming language.\n\nClick Install to continue.");
            ShowWindow(g_btnInstall, SW_SHOW);
            ShowWindow(g_btnCancel, SW_SHOW);
            break;
            
        case SetupState::Maintenance:
            SetWindowTextA(g_hTitle, "Nux Maintenance");
            SetWindowTextA(g_hDesc, "Nux is already installed on this machine.\n\nChoose an action below to repair or remove the installation.");
            ShowWindow(g_btnRepair, SW_SHOW);
            ShowWindow(g_btnUninstall, SW_SHOW);
            ShowWindow(g_btnCancel, SW_SHOW);
            break;
            
        case SetupState::Installing:
            SetWindowTextA(g_hTitle, "Installing...");
            SetWindowTextA(g_hDesc, "Please wait while Nux is being configured on your system.");
            ShowWindow(g_hProgressBar, SW_SHOW);
            ShowWindow(g_hStatusText, SW_SHOW);
            break;
            
        case SetupState::Complete:
            SetWindowTextA(g_hTitle, "Success");
            SetWindowTextA(g_hDesc, "The operation was completed successfully.\nNux is ready to use.");
            ShowWindow(g_btnFinish, SW_SHOW);
            break;
            
        case SetupState::Failed:
            SetWindowTextA(g_hTitle, "Failed");
            SetWindowTextA(g_hDesc, "The operation could not be completed. An error occurred.");
            ShowWindow(g_btnFinish, SW_SHOW);
            break;
    }
}

void ExecuteMsiAction(std::string args) {
    g_state = SetupState::Installing;
    PostMessage(g_hWnd, WM_APP, 0, 0); // Trigger UI update on main thread
    
    // Extract MSI
    HRSRC hRes = FindResource(NULL, MAKEINTRESOURCE(IDR_MSI_PAYLOAD), RT_RCDATA);
    if (!hRes) {
        g_state = SetupState::Failed;
        PostMessage(g_hWnd, WM_APP, 0, 0);
        return;
    }
    
    HGLOBAL hMem = LoadResource(NULL, hRes);
    DWORD size = SizeofResource(NULL, hRes);
    void* data = LockResource(hMem);
    
    char tempPath[MAX_PATH];
    GetTempPathA(MAX_PATH, tempPath);
    std::string msiPath = std::string(tempPath) + "nux_installer.msi";
    
    std::ofstream out(msiPath, std::ios::binary);
    out.write((char*)data, size);
    out.close();
    
    SendMessage(g_hProgressBar, PBM_SETPOS, 50, 0);
    
    std::string fullArgs = args + " \"" + msiPath + "\" /qn";
    
    SHELLEXECUTEINFOA sei = {0};
    sei.cbSize = sizeof(SHELLEXECUTEINFOA);
    sei.fMask = SEE_MASK_NOCLOSEPROCESS;
    sei.hwnd = g_hWnd;
    sei.lpVerb = "runas"; 
    sei.lpFile = "msiexec.exe";
    sei.lpParameters = fullArgs.c_str();
    sei.nShow = SW_HIDE;
    
    if (ShellExecuteExA(&sei)) {
        WaitForSingleObject(sei.hProcess, INFINITE);
        DWORD exitCode = 0;
        GetExitCodeProcess(sei.hProcess, &exitCode);
        CloseHandle(sei.hProcess);
        
        g_state = (exitCode == 0) ? SetupState::Complete : SetupState::Failed;
    } else {
        g_state = SetupState::Failed;
    }
    
    DeleteFileA(msiPath.c_str());
    PostMessage(g_hWnd, WM_APP, 0, 0); // Trigger final UI update
}

LRESULT CALLBACK WindowProc(HWND hwnd, UINT uMsg, WPARAM wParam, LPARAM lParam) {
    switch (uMsg) {
        case WM_CREATE: {
            g_hLogo = (HBITMAP)LoadImageA(GetModuleHandle(NULL), MAKEINTRESOURCEA(IDB_LOGO), IMAGE_BITMAP, 200, 350, LR_CREATEDIBSECTION);

            HFONT hFontTitle = CreateFontA(32, 0, 0, 0, FW_BOLD, FALSE, FALSE, FALSE, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, DEFAULT_PITCH | FF_SWISS, "Segoe UI");
            HFONT hFontNormal = CreateFontA(16, 0, 0, 0, FW_NORMAL, FALSE, FALSE, FALSE, ANSI_CHARSET, OUT_DEFAULT_PRECIS, CLIP_DEFAULT_PRECIS, DEFAULT_QUALITY, DEFAULT_PITCH | FF_SWISS, "Segoe UI");

            int contentX = 220; // 200 (logo) + 20 padding

            g_hTitle = CreateWindowA("STATIC", "", WS_CHILD | WS_VISIBLE, contentX, 30, 350, 40, hwnd, NULL, NULL, NULL);
            SendMessage(g_hTitle, WM_SETFONT, (WPARAM)hFontTitle, TRUE);

            g_hDesc = CreateWindowA("STATIC", "", WS_CHILD | WS_VISIBLE, contentX, 80, 350, 80, hwnd, NULL, NULL, NULL);
            SendMessage(g_hDesc, WM_SETFONT, (WPARAM)hFontNormal, TRUE);

            g_hProgressBar = CreateWindowExA(0, PROGRESS_CLASS, NULL, WS_CHILD | PBS_SMOOTH, contentX, 180, 350, 15, hwnd, NULL, NULL, NULL);
            
            g_hStatusText = CreateWindowA("STATIC", "Working...", WS_CHILD, contentX, 200, 350, 20, hwnd, NULL, NULL, NULL);
            SendMessage(g_hStatusText, WM_SETFONT, (WPARAM)hFontNormal, TRUE);

            g_btnInstall = CreateWindowA("BUTTON", "Install", WS_CHILD | BS_PUSHBUTTON | BS_FLAT, contentX, 280, 100, 35, hwnd, (HMENU)BTN_INSTALL, NULL, NULL);
            SendMessage(g_btnInstall, WM_SETFONT, (WPARAM)hFontNormal, TRUE);
            
            g_btnRepair = CreateWindowA("BUTTON", "Repair", WS_CHILD | BS_PUSHBUTTON | BS_FLAT, contentX, 280, 100, 35, hwnd, (HMENU)BTN_REPAIR, NULL, NULL);
            SendMessage(g_btnRepair, WM_SETFONT, (WPARAM)hFontNormal, TRUE);
            
            g_btnUninstall = CreateWindowA("BUTTON", "Uninstall", WS_CHILD | BS_PUSHBUTTON | BS_FLAT, contentX + 110, 280, 100, 35, hwnd, (HMENU)BTN_UNINSTALL, NULL, NULL);
            SendMessage(g_btnUninstall, WM_SETFONT, (WPARAM)hFontNormal, TRUE);

            g_btnCancel = CreateWindowA("BUTTON", "Cancel", WS_CHILD | BS_PUSHBUTTON | BS_FLAT, 470, 280, 100, 35, hwnd, (HMENU)BTN_CANCEL, NULL, NULL);
            SendMessage(g_btnCancel, WM_SETFONT, (WPARAM)hFontNormal, TRUE);

            g_btnFinish = CreateWindowA("BUTTON", "Close", WS_CHILD | BS_PUSHBUTTON | BS_FLAT, 470, 280, 100, 35, hwnd, (HMENU)BTN_FINISH, NULL, NULL);
            SendMessage(g_btnFinish, WM_SETFONT, (WPARAM)hFontNormal, TRUE);

            g_state = IsNuxInstalled() ? SetupState::Maintenance : SetupState::Welcome;
            UpdateUIState();
            return 0;
        }
        case WM_APP: { // Custom message to update UI from thread
            UpdateUIState();
            return 0;
        }
        case WM_CTLCOLORSTATIC: {
            HDC hdcStatic = (HDC)wParam;
            SetTextColor(hdcStatic, COLOR_TEXT);
            SetBkColor(hdcStatic, COLOR_BG);
            return (LRESULT)hBgBrush;
        }
        case WM_COMMAND: {
            if (LOWORD(wParam) == BTN_INSTALL) {
                std::thread(ExecuteMsiAction, "/i").detach();
            } else if (LOWORD(wParam) == BTN_REPAIR) {
                std::thread(ExecuteMsiAction, "/f").detach();
            } else if (LOWORD(wParam) == BTN_UNINSTALL) {
                std::thread(ExecuteMsiAction, "/x").detach();
            } else if (LOWORD(wParam) == BTN_CANCEL || LOWORD(wParam) == BTN_FINISH) {
                PostQuitMessage(0);
            }
            return 0;
        }
        case WM_PAINT: {
            PAINTSTRUCT ps;
            HDC hdc = BeginPaint(hwnd, &ps);
            FillRect(hdc, &ps.rcPaint, hBgBrush);
            
            if (g_hLogo) {
                HDC hdcMem = CreateCompatibleDC(hdc);
                HBITMAP hbmOld = (HBITMAP)SelectObject(hdcMem, g_hLogo);
                BitBlt(hdc, 0, 0, 200, 350, hdcMem, 0, 0, SRCCOPY);
                SelectObject(hdcMem, hbmOld);
                DeleteDC(hdcMem);
            }
            
            // Accent line top
            RECT topRect = {0, 0, 600, 4};
            HBRUSH hAccent = CreateSolidBrush(COLOR_ACCENT);
            FillRect(hdc, &topRect, hAccent);
            DeleteObject(hAccent);
            
            EndPaint(hwnd, &ps);
            return 0;
        }
        case WM_NCHITTEST: {
            LRESULT hit = DefWindowProc(hwnd, uMsg, wParam, lParam);
            if (hit == HTCLIENT) return HTCAPTION;
            return hit;
        }
        case WM_DESTROY: {
            if (g_hLogo) DeleteObject(g_hLogo);
            PostQuitMessage(0);
            return 0;
        }
    }
    return DefWindowProcA(hwnd, uMsg, wParam, lParam);
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    InitCommonControls();
    WNDCLASSA wc = {0};
    wc.lpfnWndProc = WindowProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = "NuxSetupWizard";
    wc.hCursor = LoadCursor(NULL, IDC_ARROW);
    wc.hbrBackground = hBgBrush;
    wc.hIcon = LoadIcon(hInstance, MAKEINTRESOURCE(IDI_APP_ICON));
    RegisterClassA(&wc);

    int w = 600;
    int h = 350;
    int sw = GetSystemMetrics(SM_CXSCREEN);
    int sh = GetSystemMetrics(SM_CYSCREEN);

    g_hWnd = CreateWindowExA(WS_EX_APPWINDOW | WS_EX_TOPMOST, "NuxSetupWizard", "Nux Setup",
                            WS_POPUP | WS_VISIBLE, (sw - w) / 2, (sh - h) / 2, w, h,
                            NULL, NULL, hInstance, NULL);

    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }
    return 0;
}
