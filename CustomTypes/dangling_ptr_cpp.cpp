#include<iostream>

int main() {
    int* p = nullptr;

    {
        int x = 5;
        p = &x;
    } // x is out of scope here

    std::cout << *p << std::endl; // Dangling pointer! *p is undefined behavior

    return 0;
}
