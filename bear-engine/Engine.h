#pragma once

#if defined(_WIN32) || defined(WIN32) 
#ifdef BEARENGINE_EXPORTS
#define BEARENGINE_API __declspec(dllexport)
#else
#define BEARENGINE_API __declspec(dllimport)
#endif
#elif defined(__unix__)
#define LIBQPP_API 
#endif // _WIN32

namespace bear
{
    void BEARENGINE_API HndlEngineChangeLighting(
        bool bLightOn
    );
     bool BEARENGINE_API HndlEngineGetKeyState(uint8_t bKeyCode);
     void BEARENGINE_API HndlEngineRun(bool(*FctDraw)(void));
    int32_t BEARENGINE_API HndlEngineCreateWindow();
};
