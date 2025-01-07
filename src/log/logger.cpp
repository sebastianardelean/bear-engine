#include <spdlog/spdlog.h>
#include <spdlog/sinks/basic_file_sink.h>
#include <spdlog/sinks/stdout_color_sinks.h>
#include "log/logger.hpp"

namespace bear::utils {

    // Define static members
    std::shared_ptr<spdlog::logger> Logger::instance = nullptr;
    std::once_flag Logger::initFlag;

    std::shared_ptr<spdlog::logger> Logger::GetInstance() {
        std::call_once(initFlag, []() {
            // Initialize the logger once
            try {
                auto consoleSink = std::make_shared<spdlog::sinks::stdout_color_sink_mt>();
                auto fileSink = std::make_shared<spdlog::sinks::basic_file_sink_mt>("logs.txt", true);

                // Combine both sinks into a logger
                spdlog::set_pattern("[%Y-%m-%d %H:%M:%S] [%l] [Thread %t] %v");
                instance = std::make_shared<spdlog::logger>("bear", spdlog::sinks_init_list{ consoleSink, fileSink });
                instance->set_level(spdlog::level::info); // Set the default log level
            }
            catch (const spdlog::spdlog_ex& ex) {
                std::println("Log initialization failed: {}", ex.what());
            }
            });

        return instance;
    }


        
}