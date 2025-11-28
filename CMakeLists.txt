cmake_minimum_required(VERSION 3.28)

project(cmake_project_template)

# C++ settings
set(CMAKE_CXX_STANDARD 23)
set(CMAKE_CXX_STANDARD_REQUIRED ON)

# Final executable
add_executable(${PROJECT_NAME} src/main.cpp)