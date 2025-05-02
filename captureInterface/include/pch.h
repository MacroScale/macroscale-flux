// pch.h
#ifndef PCH_H
#define PCH_H

// Include unknwn.h before any WinRT headers
#include <windows.h>   
#include <unknwn.h>

// WinRT
#include <winrt/Windows.Foundation.h>
#include <winrt/Windows.System.h>
#include <winrt/Windows.Graphics.Capture.h>
#include <winrt/Windows.Graphics.DirectX.h>
#include <winrt/Windows.Graphics.DirectX.Direct3D11.h>

#include <dispatcherqueue.h>

// D3D
#include <d3d11_4.h>
#include <dxgi1_6.h>
#include <d2d1_3.h>
#include <wincodec.h>

// Helpers
#include "direct3d11.interop.h"

#endif
