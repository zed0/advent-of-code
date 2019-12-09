#include <iostream>

int main()
{
	int a = 1; // 0
	int b = 0; // 1
	int c = 0; // 2
	int d = 0; // 3
	int e = 0; // 4
	int g = 0; // 6

	b += 4;
	b *= 19;
	b *= 11;
	e += 7;
	e *= 22;
	e += 20;
	b += e;
	if(a)
	{
		e = 27;
		e *= 28;
		e += 29;
		e *= 30;
		e *= 14;
		e *= 32;
		b += e;
	}

	std::cout << __FILE__ << ":" << __LINE__ << ": Value of b: " << b << std::endl;
}
