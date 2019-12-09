#include <iostream>
#include <string>
#include <sstream>

int clamp(int in)
{
	if(in > 2)
		return 2;
	if(in < 0)
		return 0;
	return in;
}

int to_code(int x, int y)
{
	return 3*y + x + 1;
}

int main()
{
	int x=1;
	int y=1;

	std::string line;
	while(std::getline(std::cin, line))
	{
		std::istringstream iss(line);
		char dir;
		while(iss >> dir)
		{
			switch(dir)
			{
				case 'U':
					y = clamp(y-1); break;
				case 'D':
					y = clamp(y+1); break;
				case 'L':
					x = clamp(x-1); break;
				case 'R':
					x = clamp(x+1); break;
			}
		}
		std::cout << x << ',' << y << std::endl;
		std::cout << to_code(x, y) << std::endl;

	}
}
