// Two overloads: same name in the source, different symbols in the object.
int add(int a, int b) { return a + b; }
double add(double a, double b) { return a + b; }

// extern "C" turns mangling off: this one is callable from anywhere.
extern "C" int c_add(int a, int b) { return a + b; }
