#include "core/auto_release.hpp"

#include "log/logger.hpp"
#include "core/engine.hpp"
#include "core/context.hpp"


static bear::Context g_Context;

namespace bear {
	

	

	auto EngineInitialize(std::string& name, int32_t winWidth, int32_t winHeight) -> void
	{
		
		::g_Context = Context( name, winWidth, winHeight );

		// clear the renderer
		SDL_SetRenderDrawColor(::g_Context.g_pRenderer.get(), 0, 0, 0, 0);
		SDL_RenderClear(::g_Context.g_pRenderer.get());
		SDL_RenderPresent(::g_Context.g_pRenderer.get());
	}

	
	auto EngineRun(std::function<void(Context &ctx)> hndlFunct) -> void
	{
		
		bool done = false;

		ImVec4 clear_color = ImVec4(0.45f, 0.55f, 0.60f, 1.00f);

		IMGUI_CHECKVERSION();
		ImGui::CreateContext();
		ImGuiIO& io = ImGui::GetIO(); (void)io;
		io.ConfigFlags |= ImGuiConfigFlags_NavEnableKeyboard;     // Enable Keyboard Controls



		// Setup Dear ImGui style
		ImGui::StyleColorsDark();
		//ImGui::StyleColorsLight();

		// Setup Platform/Renderer backends

		ImGui_ImplSDL2_InitForSDLRenderer(::g_Context.g_pWindow.get(), ::g_Context.g_pRenderer.get());
		ImGui_ImplSDLRenderer2_Init(::g_Context.g_pRenderer.get());

		
		while(!done) {

			SDL_Event event;
		
			//Handle events on queue
			while (SDL_PollEvent(&event) != 0)
			{
				ImGui_ImplSDL2_ProcessEvent(&event);
				switch (event.type)
				{
				case ::SDL_QUIT:
				case ::SDL_WINDOWEVENT_CLOSE:
					done = true;
					break;
				case ::SDL_WINDOWEVENT:
					if (event.window.event == SDL_WINDOWEVENT_RESIZED) {
						SDL_GetWindowSize(::g_Context.g_pWindow.get(), &::g_Context.g_i32WinWidth, &::g_Context.g_i32WinHeight);
					}
					break;
				
					
				
				default:
					break;
				}
				
			}
			


			ImGui_ImplSDLRenderer2_NewFrame();
			ImGui_ImplSDL2_NewFrame();
			ImGui::NewFrame();
			ImGui::ShowDemoWindow();

			{
				static float f = 0.0f;
				static int counter = 0;

				ImGui::Begin("Hello, world!");                          // Create a window called "Hello, world!" and append into it.

				ImGui::Text("This is some useful text.");               // Display some text (you can use a format strings too)


				ImGui::SliderFloat("float", &f, 0.0f, 1.0f);            // Edit 1 float using a slider from 0.0f to 1.0f
				ImGui::ColorEdit3("clear color", (float*)&clear_color); // Edit 3 floats representing a color

				if (ImGui::Button("Button"))                            // Buttons return true when clicked (most widgets return true when edited/activated)
					counter++;
				ImGui::SameLine();
				ImGui::Text("counter = %d", counter);

				ImGui::Text("Application average %.3f ms/frame (%.1f FPS)", 1000.0f / io.Framerate, io.Framerate);
				ImGui::End();
				ImGui::EndFrame();

			}

			SDL_SetRenderDrawColor(::g_Context.g_pRenderer.get(), 255, 255, 255, 255);
			SDL_RenderClear(::g_Context.g_pRenderer.get());

			hndlFunct(::g_Context);
			

			// Rendering
			ImGui::Render();
			SDL_RenderSetScale(::g_Context.g_pRenderer.get(), io.DisplayFramebufferScale.x, io.DisplayFramebufferScale.y);
			
			ImGui_ImplSDLRenderer2_RenderDrawData(ImGui::GetDrawData(), ::g_Context.g_pRenderer.get());
			SDL_RenderPresent(::g_Context.g_pRenderer.get());
		}

		ImGui_ImplSDLRenderer2_Shutdown();
		ImGui_ImplSDL2_Shutdown();
		ImGui::DestroyContext();
	

	}
	
}