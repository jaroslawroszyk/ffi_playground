#include <string>
#include <iostream>

void hello_from_cpp(const std::string& name){
    std::cout << "Hello from C++, " << name << "!" << std::endl;
}

extern "C" {
    void hello_from_c(const char* name) {
        hello_from_cpp(std::string(name));
    }
}