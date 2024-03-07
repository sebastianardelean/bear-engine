/**
 * Automatic tilling window manager for windows
 * Inspired from https://github.com/nir9/lightwm/tree/master
 */
#include <Windows.h>

#include <cstdlib>
#include <csignal>

#include <string>
#include <engine_types.h>
#include <configuration.h>






static void HndlSIGINT(int signal)
{

    exit(0);
}

static bool HndlDraw(void)
{
    
    if (bear::HndlEngineGetKeyState(VK_F2))
    {

        bear::HndlEngineChangeLighting(true);
        
    }
    if (bear::HndlEngineGetKeyState(VK_F3))
    {

        bear::HndlEngineChangeLighting(false);
        
    }
    if (bear::HndlEngineGetKeyState(VK_F1))
    {
#if 0
        bear::Sprite spr = bear::Sprite(100, 100, L"c:\\Users\\sardelean\\Documents\\workspace\\bear-graphic-engine\\NeHe.bmp");
        spr.Draw();
#endif
#if 0
        size_t i = 0;
        size_t j = 0;
        for (i = 0; i < SCREEN_WIDTH; i++)
        {
            for (j = 0; j < SCREEN_HEIGHT; j++)
            {
                coordinate_t p = { (float)i,(float)j, -1.0f };
                color_t color = { rand() % 255u, rand() % 255u, rand() % 255u, 0 };
                bear::Point ptr = bear::Point(p, color);
                ptr.Draw(false);
            }
        }

#endif
#if 1

        coordinate_t p1 = { 0.0f, 0.0f, -1.0f };
        coordinate_t p2 = { 200.0f, 200.0f, -1.0f };
        color_t color = { rand() % 255, rand() % 255, rand() % 255, 0 };
        bear::Line line = bear::Line(p1, p2, color);
        line.Draw(false);


        coordinate_t pr1 = { 200.0f, 200.0f, -2.0f };
        coordinate_t pr2 = { 250.0f, 200.0f, -2.0f };
        coordinate_t pr3 = { 200.0f, 150.0f, -2.0f };

        color_t cr;
        cr.r = 255;
        cr.g = rand() % 255;
        cr.b = rand() % 255;
        cr.a = 0;
    


        fill_option_t fill_option;
        fill_option.fill_type = FILL_COLOR;
        fill_option.color = cr;

        bear::Triangle tr = bear::Triangle(pr1, pr2, pr3, fill_option);
        tr.Draw(false);

        
       



#endif
        return TRUE;
    }
    return FALSE;
}

int main(void)
{

    signal(SIGINT, HndlSIGINT);

    if (bear::HndlEngineCreateWindow() != 0)
    {
        return -1;
    }

    bear::HndlEngineRun(&HndlDraw);


    return EXIT_SUCCESS;
}
