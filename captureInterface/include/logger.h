// Logger.h
#ifndef LOGGER_H
#define LOGGER_H

#include <ctime>
#include <fstream>
#include <iostream>
#include <sstream>

// Enum to represent log levels
enum LogLevel { DEBUG, INFO, WARNING, ERROR, CRITICAL };

class Logger {
public:
    // Constructor: Opens the log file in append mode
    static Logger& instance(const std::string& filename)
    {
        static Logger inst;
        inst.logFile.open(filename, std::ios::app);
        if (!inst.logFile.is_open()) {
            std::cerr << "Error opening log file." << std::endl;
        }
        return inst; 
    }

    // Destructor: Closes the log file
    ~Logger() { logFile.close(); }

    // Logs a message with a given log level
    void log(LogLevel level, const std::string& message) {
        // Get current timestamp
        time_t now = time(0);
        tm* timeinfo = localtime(&now);
        char timestamp[20];
        strftime(timestamp, sizeof(timestamp), "%Y-%m-%d %H:%M:%S", timeinfo);

        // Create log entry
        std::ostringstream logEntry;
        logEntry << "[" << timestamp << "] "
                 << levelToString(level) << ": " << message
                 << std::endl;

        // Output to console
        std::cout << logEntry.str();

        // Output to log file
        if (logFile.is_open()) {
            logFile << logEntry.str();
            logFile
                .flush(); // Ensure immediate write to file
        }
    }

    void info(const std::string& message){
        log(INFO, message);
    }
    void error(const std::string& message){
        log(ERROR, message);
    }


private:
    Logger() = default;

    std::ofstream logFile; 

    std::string levelToString(LogLevel level)
    {
        switch (level) {
        case DEBUG: return "DEBUG";
        case INFO: return "INFO";
        case WARNING: return "WARNING";
        case ERROR: return "ERROR";
        case CRITICAL: return "CRITICAL";
        default: return "UNKNOWN";
        }
    }
    // make sure these are inaccessible(especially from outside), 
    // otherwise, you may accidentally get copies of the singleton appearing.
    Logger(Logger const&) = delete;
    void operator=(Logger const&) = delete;

};

/*int main()*/
/*{*/
/*    // Example usage of the logger*/
/*    logger.log(INFO, "Program started.");*/
/*    logger.log(DEBUG, "Debugging information.");*/
/*    logger.log(ERROR, "An error occurred.");*/
/**/
/*    return 0;*/
/*}*/
/**/

// Make SLOG a reference to the singleton instance
static Logger& SLOG = Logger::instance("log.txt");

#endif
