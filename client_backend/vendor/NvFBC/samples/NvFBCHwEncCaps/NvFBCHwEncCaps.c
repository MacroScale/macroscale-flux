/*!
 * \brief
 * Demonstrates how to use NvFBC to query the capabilities of the HW encoder.
 *
 * \file
 * This sample demonstrates the following features:
 * - Query the HW encoder capabilities.
 *
 * \copyright
 * Copyright (c) 2013-2015, NVIDIA CORPORATION. All rights reserved.
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice shall be included in
 * all copies or substantial portions of the Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
 * DEALINGS IN THE SOFTWARE.
 */

#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <dlfcn.h>
#include <string.h>
#include <getopt.h>

#include <NvFBC.h>

#include "NvFBCUtils.h"

#define APP_VERSION 1

#define LIB_NVFBC_NAME "libnvidia-fbc.so.1"

/**
 * Prints usage information.
 */
static void usage(const char *pname)
{
    printf("Usage: %s [options]\n", pname);
    printf("\n");
    printf("Options:\n");
    printf("  --help|-h         This message\n");
    printf("  --codec|-c <str>  Codec to use\n");
    printf("                    Can be 'h264' or 'hevc'\n");
    printf("                    (default: 'h264')\n");
}


/**
 * Initializes the NvFBC library and creates an NvFBC instance.
 */
int main(int argc, char *argv[])
{
    static struct option longopts[] = {
        { "codec", required_argument, NULL, 'c' },
        { NULL, 0, NULL, 0 }
    };

    int opt;

    void *libNVFBC = NULL;
    PNVFBCCREATEINSTANCE NvFBCCreateInstance_ptr = NULL;
    NVFBC_API_FUNCTION_LIST pFn;

    NVFBCSTATUS fbcStatus;

    NVFBC_SESSION_HANDLE fbcHandle;
    NVFBC_CREATE_HANDLE_PARAMS createHandleParams;
    NVFBC_CREATE_CAPTURE_SESSION_PARAMS createCaptureParams;
    NVFBC_TOHWENC_GET_CAPS_PARAMS getCapsParams;
    NVFBC_DESTROY_CAPTURE_SESSION_PARAMS destroyCaptureParams;
    NVFBC_DESTROY_HANDLE_PARAMS destroyHandleParams;

    NVFBC_HWENC_CODEC codec = NVFBC_HWENC_CODEC_H264;

    /*
     * Parse the command line.
     */
    while ((opt = getopt_long(argc, argv, "hc:", longopts, NULL)) != -1) {
        switch (opt) {
            case 'c':
                if (!strcasecmp(optarg, "h264")) {
                    codec = NVFBC_HWENC_CODEC_H264;
                } else if (!strcasecmp(optarg, "hevc")) {
                    codec = NVFBC_HWENC_CODEC_HEVC;
                } else {
                    fprintf(stderr, "Invalid codec: '%s'\n", optarg);
                    return EXIT_FAILURE;
                }
                break;
            case 'h':
            default:
                usage(argv[0]);
                return EXIT_SUCCESS;
        }
    }

    NvFBCUtilsPrintVersions(APP_VERSION);

    /*
     * Dynamically load the NvFBC library.
     */
    libNVFBC = dlopen(LIB_NVFBC_NAME, RTLD_NOW);
    if (libNVFBC == NULL) {
        fprintf(stderr, "Unable to open '%s' (%s)\n", LIB_NVFBC_NAME, dlerror());
        return EXIT_FAILURE;
    }

    /*
     * Resolve the 'NvFBCCreateInstance' symbol that will allow us to get
     * the API function pointers.
     */
    NvFBCCreateInstance_ptr =
        (PNVFBCCREATEINSTANCE) dlsym(libNVFBC, "NvFBCCreateInstance");
    if (NvFBCCreateInstance_ptr == NULL) {
        fprintf(stderr, "Unable to resolve symbol 'NvFBCCreateInstance'\n");
        return EXIT_FAILURE;
    }

    /*
     * Create an NvFBC instance.
     *
     * API function pointers are accessible through pFn.
     */
    memset(&pFn, 0, sizeof(pFn));

    pFn.dwVersion = NVFBC_VERSION;

    fbcStatus = NvFBCCreateInstance_ptr(&pFn);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "Unable to create NvFBC instance (status: %d)\n",
                fbcStatus);
        return EXIT_FAILURE;
    }

    /*
     * Create a session handle that is used to identify the client.
     */
    memset(&createHandleParams, 0, sizeof(createHandleParams));

    createHandleParams.dwVersion = NVFBC_CREATE_HANDLE_PARAMS_VER;

    fbcStatus = pFn.nvFBCCreateHandle(&fbcHandle, &createHandleParams);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "%s\n", pFn.nvFBCGetLastErrorStr(fbcHandle));
        return EXIT_FAILURE;
    }

    /*
     * Create a capture session.
     */
    memset(&createCaptureParams, 0, sizeof(createCaptureParams));

    createCaptureParams.dwVersion     = NVFBC_CREATE_CAPTURE_SESSION_PARAMS_VER;
    createCaptureParams.eCaptureType  = NVFBC_CAPTURE_TO_HW_ENCODER;

    fbcStatus = pFn.nvFBCCreateCaptureSession(fbcHandle, &createCaptureParams);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "%s\n", pFn.nvFBCGetLastErrorStr(fbcHandle));
        return EXIT_FAILURE;
    }

    /*
     * Retrieve the capabilities of the HW encoder.
     */
    memset(&getCapsParams, 0, sizeof(getCapsParams));

    getCapsParams.dwVersion = NVFBC_TOHWENC_GET_CAPS_PARAMS_VER;
    getCapsParams.codec     = codec;

    fbcStatus = pFn.nvFBCToHwEncGetCaps(fbcHandle, &getCapsParams);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "%s\n", pFn.nvFBCGetLastErrorStr(fbcHandle));
        return EXIT_FAILURE;
    }

#define YN(cond) ((cond) ? "Yes" : "No")

    printf("HW encoder capabilities for ");
    if (codec == NVFBC_HWENC_CODEC_H264) {
        printf("H.264:\n");
    } else {
        printf("HEVC/H.265:\n");
    }

    printf("- Codec support: %s\n", YN(getCapsParams.bCodecSupported));

    printf("- YUV444 support: %s\n", YN(getCapsParams.bYUV444));
    printf("- Lossless support: %s\n", YN(getCapsParams.bLossless));

    printf("- Maximum frame resolution: %ux%u\n",
           getCapsParams.dwMaxWidth, getCapsParams.dwMaxHeight);
    printf("- Maximum frame size: %uMB\n", getCapsParams.dwMaxMB);
    printf("- Maximum throughput: %uMB/s\n", getCapsParams.dwMaxMBPerSec);

    printf("- Rate control capabilities:\n");
    printf("  - Constant QP: %s\n", YN(getCapsParams.bRcConstQP));
    printf("  - Variable bitrate mode: %s\n", YN(getCapsParams.bRcVbr));
    printf("  - Constant bitrate mode: %s\n", YN(getCapsParams.bRcCbr));
    printf("  - Multi pass for image quality: %s\n", YN(getCapsParams.bRc2PassQuality));
    printf("  - Multi pass for constant frame size: %s\n", YN(getCapsParams.bRc2PassFramesizeCap));

    printf("- Dynamic resolution change: %s\n", YN(getCapsParams.bDynResChange));
    printf("- Dynamic bitrate change: %s\n", YN(getCapsParams.bDynBitrateChange));

    printf("- Intra refresh: %s\n", YN(getCapsParams.bIntraRefresh));
    printf("- Custom VBV buffer size: %s\n", YN(getCapsParams.bCustomVBVBufSize));

    /*
     * Destroy capture session, tear down resources.
     */
    memset(&destroyCaptureParams, 0, sizeof(destroyCaptureParams));

    destroyCaptureParams.dwVersion = NVFBC_DESTROY_CAPTURE_SESSION_PARAMS_VER;

    fbcStatus = pFn.nvFBCDestroyCaptureSession(fbcHandle, &destroyCaptureParams);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "%s\n", pFn.nvFBCGetLastErrorStr(fbcHandle));
        return EXIT_FAILURE;
    }

    /*
     * Destroy session handle, tear down more resources.
     */
    memset(&destroyHandleParams, 0, sizeof(destroyHandleParams));

    destroyHandleParams.dwVersion = NVFBC_DESTROY_HANDLE_PARAMS_VER;

    fbcStatus = pFn.nvFBCDestroyHandle(fbcHandle, &destroyHandleParams);
    if (fbcStatus != NVFBC_SUCCESS) {
        fprintf(stderr, "%s\n", pFn.nvFBCGetLastErrorStr(fbcHandle));
        return EXIT_FAILURE;
    }

    return EXIT_SUCCESS;
}
