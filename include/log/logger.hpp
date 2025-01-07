#pragma once

namespace bear::utils {
class Logger
{
public:

    static std::shared_ptr<spdlog::logger> GetInstance();
    
    
protected:

private:

       
    Logger() = default;
    ~Logger() = default;
    Logger(const Logger& rs) = delete;
    Logger(Logger&& rs) = delete;
    Logger& operator = (const Logger& rs) = delete;
    Logger& operator = (Logger&& rs) = delete;

    
    
    static std::shared_ptr<spdlog::logger> instance;
    static std::once_flag initFlag; // Ensures initialization happens only once
};

}