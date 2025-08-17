#include <iostream>
#include <spdlog/spdlog.h>

int main()
{
	// Set the global log level to info to see messages from info and above.
	spdlog::set_level(spdlog::level::info);

	// Log a few messages at different levels to demonstrate spdlog.
	spdlog::info("Welcome to the spdlog console application!");
	spdlog::warn("This is a warning message.");
	spdlog::error("This is an error message.");
	spdlog::critical("This is a critical message!");

	// Log a formatted message using the "fmt" syntax included with spdlog.
	spdlog::info("The answer is {}", 42);

	return 0;
}