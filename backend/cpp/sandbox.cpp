#include <iostream>

extern "C" void start_sandbox()
{
    std::cout << "Sandbox initialized!" << std::endl;
}
